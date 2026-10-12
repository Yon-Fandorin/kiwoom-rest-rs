//! 실행 시 선택한 전송과 명시적으로 표시하는 합성 데모 응답입니다.
use async_trait::async_trait;
use kiwoom_client::{
    core::ClientError,
    rest::{HttpRequest, HttpResponse, Transport, TransportError},
};
use serde_json::json;
use std::{collections::BTreeMap, sync::Arc};

/// 앱의 한 서버에서 동일한 SDK 클라이언트와 전송을 공유합니다.
#[derive(Clone)]
pub struct BackendTransport(Arc<dyn Transport>);

impl BackendTransport {
    /// 데모·네트워크·테스트 전송을 같은 도구 처리기로 조합합니다.
    pub fn new(transport: impl Transport + 'static) -> Self {
        Self(Arc::new(transport))
    }
}

#[async_trait]
impl Transport for BackendTransport {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.0.execute(request).await
    }

    async fn execute_checked(&self, request: HttpRequest) -> Result<HttpResponse, ClientError> {
        self.0.execute_checked(request).await
    }
}

/// 외부 I/O 없이 합성 데이터만 반환하며 실제 시세로 사용할 수 없습니다.
pub struct DemoTransport;

#[async_trait]
impl Transport for DemoTransport {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        let mut body = match request.api_id {
            "ka10001" => {
                json!({"stk_cd": request.body["stk_cd"], "stk_nm": "합성 테스트 종목", "cur_prc": "+000001.20"})
            }
            "ka10004" => {
                json!({"bid_req_base_tm": "120000", "sel_fpr_bid": "+000001.30", "buy_fpr_bid": "+000001.10"})
            }
            "ka10099" => {
                json!({"list": [{"code": "000001", "name": "합성 테스트 종목", "lastPrice": "+000001.20"}]})
            }
            "ka10081" => {
                json!({
                    "stk_cd": request.body["stk_cd"],
                    "stk_dt_pole_chart_qry": [{
                        "dt": request.body["base_dt"],
                        "cur_prc": "+000001.20",
                        "trde_qty": "000010"
                    }]
                })
            }
            "kt00018" => {
                json!({"tot_pur_amt": "000001.00", "tot_evlt_amt": "000001.20", "acnt_evlt_remn_indv_tot": []})
            }
            _ => return Err(TransportError),
        };
        body["return_code"] = json!(0);
        Ok(HttpResponse {
            status: 200,
            headers: BTreeMap::from([
                ("cont-yn".into(), "N".into()),
                ("api-id".into(), request.api_id.into()),
            ]),
            body,
        })
    }
}
