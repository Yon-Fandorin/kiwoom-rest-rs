//! MCP 입력을 SDK의 typed 요청으로 변환하고 근거가 있는 원본 결과를 반환합니다.
use crate::{BackendTransport, ToolPolicy};
use kiwoom_client::{
    core::{ClientError, Page, Secret},
    domain::{
        accounts::Kt00018Request,
        charts::Ka10081Request,
        market::{Ka10001Request, Ka10004Request, Ka10099Request, StockListMarket},
    },
    rest::{Client, ClientOptions, WritePolicy},
};
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock, Implementation, ServerCapabilities, ServerConfig},
    schemars::{self, JsonSchema},
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{sync::Arc, time::Duration};

const MAX_RESULT_BYTES: usize = 128 * 1024;
const CALL_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct StockInput {
    /// 브로커 종목 코드. 접미사를 포함한 원문을 그대로 전달합니다.
    #[schemars(length(min = 1, max = 20))]
    stk_cd: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ListInput {
    /// 시장 구분: 0, 10, 30, 50, 60, 70, 80, 90, 2, 3, 4, 5, 6, 7, 8, 9.
    #[schemars(schema_with = "market_schema")]
    mrkt_tp: StockListMarket,
}

fn market_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "enum": [
            "0", "10", "30", "50", "60", "70", "80", "90",
            "2", "3", "4", "5", "6", "7", "8", "9"
        ]
    })
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ChartInput {
    /// 브로커 종목 코드 원문.
    #[schemars(length(min = 1, max = 20))]
    stk_cd: String,

    /// 기준일. 실제 달력의 YYYYMMDD 형식이며 생략된 날짜를 추정하지 않습니다.
    #[schemars(length(equal = 8))]
    base_dt: String,

    /// 수정주가 구분: 0 또는 1.
    upd_stkpc_tp: Adjustment,
}

#[derive(Deserialize, JsonSchema)]
enum Adjustment {
    #[serde(rename = "0")]
    Raw,

    #[serde(rename = "1")]
    Adjusted,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct AccountInput {
    /// 조회 구분: 1(합산), 2(개별).
    qry_tp: AccountQuery,

    /// 국내 거래소: KRX 또는 NXT.
    dmst_stex_tp: Exchange,
}

#[derive(Deserialize, JsonSchema)]
enum AccountQuery {
    #[serde(rename = "1")]
    Total,

    #[serde(rename = "2")]
    Individual,
}

#[derive(Deserialize, JsonSchema)]
enum Exchange {
    #[serde(rename = "KRX")]
    Krx,

    #[serde(rename = "NXT")]
    Nxt,
}

/// 한 자격 증명 범위에서 조회 도구와 SDK 호출 간격을 공유하는 서버 처리기.
#[derive(Clone)]
pub struct KiwoomMcp {
    client: Arc<Client<BackendTransport>>,
    token: Secret,
    backend: &'static str,

    tools: ToolRouter<Self>,

    calls: Arc<tokio::sync::Semaphore>,
    shutdown: tokio_util::sync::CancellationToken,
}

impl KiwoomMcp {
    /// 전송과 토큰을 주입합니다. 금융 변경을 허용하는 설정은 받지 않습니다.
    pub fn new(
        transport: BackendTransport,
        token: Secret,
        demo: bool,
    ) -> Result<Self, ClientError> {
        let client = Client::new(
            transport,
            ClientOptions {
                max_pages: 4,
                max_read_retries: 0,
                writes: WritePolicy::Deny,
                ..ClientOptions::default()
            },
        )?;
        Ok(Self {
            client: Arc::new(client),
            token,
            backend: if demo { "synthetic_demo" } else { "broker" },

            tools: Self::tool_router(),

            calls: Arc::new(tokio::sync::Semaphore::new(8)),
            shutdown: tokio_util::sync::CancellationToken::new(),
        })
    }

    /// 설정에서 제외한 도구는 발견 목록과 직접 호출 양쪽에서 제거합니다.
    pub fn with_tool_policy(mut self, policy: ToolPolicy) -> Self {
        for tool in self.tools.list_all() {
            if !policy.allows(&tool.name) {
                self.tools.remove_route(&tool.name);
            }
        }
        self
    }

    pub(crate) fn with_shutdown_token(
        mut self,
        token: tokio_util::sync::CancellationToken,
    ) -> Self {
        self.shutdown = token;
        self
    }

    async fn result<R: Serialize>(
        &self,
        api_id: &str,
        source_url: &str,
        context: &RequestContext<RoleServer>,
        operation: impl Future<Output = Result<Vec<Page<R>>, ClientError>>,
    ) -> Result<CallToolResult, ErrorData> {
        let Ok(_permit) = self.calls.try_acquire() else {
            return Ok(CallToolResult::error(vec![ContentBlock::text(
                "동시 조회 한도 8개에 도달했습니다. 실행 중인 조회가 끝난 뒤 다시 요청하세요.",
            )]));
        };
        let session_cancellation = context
            .extensions
            .get::<axum::http::request::Parts>()
            .and_then(|parts| parts.extensions.get::<crate::http::RequestCancellation>())
            .map(|signal| signal.0.clone())
            .unwrap_or_default();
        let outcome = tokio::select! {
            biased;
            _ = self.shutdown.cancelled() => {
                return Ok(CallToolResult::error(vec![ContentBlock::text("서버 종료로 조회가 취소되었습니다.")]));
            }
            _ = session_cancellation.cancelled() => {
                return Ok(CallToolResult::error(vec![ContentBlock::text("세션 종료로 조회가 취소되었습니다.")]));
            }
            _ = context.ct.cancelled() => {
                return Ok(CallToolResult::error(vec![ContentBlock::text("조회가 취소되었습니다.")]));
            }
            outcome = tokio::time::timeout(CALL_TIMEOUT, operation) => outcome,
        };
        let pages = match outcome {
            Ok(Ok(pages)) => pages,
            Ok(Err(error)) => {
                return Ok(CallToolResult::error(vec![ContentBlock::text(
                    error.to_string(),
                )]));
            }
            Err(_) => {
                return Ok(CallToolResult::error(vec![ContentBlock::text(
                    "조회 제한 시간 30초를 초과했습니다. 결과를 완전한 조회로 간주하지 마세요.",
                )]));
            }
        };
        let pages = pages
            .into_iter()
            .map(|page| {
                json!({
                    "typed": page.typed,
                    "raw": page.raw,
                    "response_headers": page.response_headers
                })
            })
            .collect::<Vec<_>>();
        let value = json!({
            "api_id": api_id,
            "source_url": source_url,
            "backend": self.backend,
            "live_verified": false,
            "complete": true,
            "pages": pages
        });
        let result = CallToolResult::structured(value);
        let size = serde_json::to_vec(&result)
            .map_err(|_| ErrorData::internal_error("결과를 직렬화하지 못했습니다.", None))?
            .len();
        if size > MAX_RESULT_BYTES {
            return Ok(CallToolResult::error(vec![ContentBlock::text(
                "조회 결과가 128 KiB 상한을 초과했습니다. 범위를 줄이세요. 결과를 자르거나 완전한 조회로 반환하지 않습니다.",
            )]));
        }
        Ok(result)
    }
}

#[tool_router]
impl KiwoomMcp {
    #[tool(
        name = "kiwoom_stock_info",
        description = "국내 주식 기본정보(ka10001)를 조회합니다. 원본 금액 문자열과 미확인 필드를 보존합니다. 데모 결과는 합성 데이터입니다.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn stock_info(
        &self,
        Parameters(input): Parameters<StockInput>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        self.result(
            "ka10001",
            "https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10001",
            &context,
            self.client.ka10001(
                &self.token,
                &Ka10001Request {
                    stk_cd: input.stk_cd,
                },
            ),
        )
        .await
    }

    #[tool(
        name = "kiwoom_order_book",
        description = "국내 주식 호가(ka10004)를 조회합니다. 원본 가격·수량 문자열을 보존합니다.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn order_book(
        &self,
        Parameters(input): Parameters<StockInput>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        self.result(
            "ka10004",
            "https://openapi.kiwoom.com/guide/apiGuideContents/02/ka10004",
            &context,
            self.client.ka10004(
                &self.token,
                &Ka10004Request {
                    stk_cd: input.stk_cd,
                },
            ),
        )
        .await
    }

    #[tool(
        name = "kiwoom_stock_list",
        description = "시장별 종목 목록(ka10099)을 조회합니다. 최대 4페이지를 모두 조회하지 못하면 오류를 반환합니다.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn stock_list(
        &self,
        Parameters(input): Parameters<ListInput>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        self.result(
            "ka10099",
            "https://openapi.kiwoom.com/guide/apiGuideContents/01/ka10099",
            &context,
            self.client.ka10099(
                &self.token,
                &Ka10099Request {
                    mrkt_tp: input.mrkt_tp,
                },
            ),
        )
        .await
    }

    #[tool(
        name = "kiwoom_daily_chart",
        description = "국내 주식 일봉(ka10081)을 지정한 기준일로 조회합니다. 가격과 거래량을 금융 값으로 추정 변환하지 않습니다.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn daily_chart(
        &self,
        Parameters(input): Parameters<ChartInput>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let request = Ka10081Request {
            stk_cd: input.stk_cd,
            base_dt: input.base_dt,
            upd_stkpc_tp: match input.upd_stkpc_tp {
                Adjustment::Raw => "0",
                Adjustment::Adjusted => "1",
            }
            .into(),
        };
        self.result(
            "ka10081",
            "https://openapi.kiwoom.com/guide/apiGuideContents/07/ka10081",
            &context,
            self.client.ka10081(&self.token, &request),
        )
        .await
    }

    #[tool(
        name = "kiwoom_account_evaluation",
        description = "국내 계좌평가 잔고내역(kt00018)을 조회합니다. 계좌 정보가 반환되므로 호출자가 결과 접근 범위를 관리해야 합니다.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn account_evaluation(
        &self,
        Parameters(input): Parameters<AccountInput>,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let request = Kt00018Request {
            qry_tp: match input.qry_tp {
                AccountQuery::Total => "1",
                AccountQuery::Individual => "2",
            }
            .into(),
            dmst_stex_tp: match input.dmst_stex_tp {
                Exchange::Krx => "KRX",
                Exchange::Nxt => "NXT",
            }
            .into(),
        };
        self.result(
            "kt00018",
            "https://openapi.kiwoom.com/guide/apiGuideContents/08/kt00018",
            &context,
            self.client.kt00018(&self.token, &request),
        )
        .await
    }
}

#[tool_handler(router = self.tools)]
impl ServerHandler for KiwoomMcp {
    fn get_info(&self) -> ServerConfig {
        let instructions = format!(
            concat!(
                "키움 비공식 조회 전용 MCP. backend={}. ",
                "금융 변경·OAuth 발급·WS 구독 도구는 제공하지 않습니다. ",
                "모든 결과의 live_verified는 false이며 데모 데이터는 실제 시세가 아닙니다."
            ),
            self.backend
        );

        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("kiwoom-mcp", env!("CARGO_PKG_VERSION")))
            .with_instructions(instructions)
    }
}
