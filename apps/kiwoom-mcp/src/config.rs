//! 명시적으로 선택한 사용자 프로필의 도구 허용 목록입니다. 자격 증명은 읽지 않습니다.
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Read,
    path::Path,
};

const MAX_CONFIG_BYTES: u64 = 64 * 1024;
const TOOLS: [&str; 5] = [
    "kiwoom_stock_info",
    "kiwoom_order_book",
    "kiwoom_stock_list",
    "kiwoom_daily_chart",
    "kiwoom_account_evaluation",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Profiles {
    version: u32,
    profiles: BTreeMap<String, Profile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    tools: Vec<String>,
}

/// 목록 표시와 호출 실행에 동일하게 적용하는 불변 허용 목록.
#[derive(Clone)]
pub struct ToolPolicy {
    allowed: BTreeSet<String>,
}

impl Default for ToolPolicy {
    fn default() -> Self {
        Self {
            allowed: TOOLS.into_iter().map(String::from).collect(),
        }
    }
}

impl ToolPolicy {
    /// 이름이 정확히 일치하는 도구만 허용합니다. 빈 목록은 모든 호출을 차단합니다.
    pub fn allow_only(tools: Vec<String>) -> Result<Self, &'static str> {
        let mut allowed = BTreeSet::new();
        for tool in tools {
            if !TOOLS.contains(&tool.as_str()) {
                return Err("허용 목록에 지원하지 않는 MCP 도구가 있습니다.");
            }
            if !allowed.insert(tool) {
                return Err("허용 목록에 도구 이름이 중복되었습니다.");
            }
        }
        Ok(Self { allowed })
    }

    /// 명시적으로 제공된 설정만 읽고, 누락·오류가 있으면 전체 허용으로 대체하지 않습니다.
    pub fn from_file(path: &Path, profile: &str) -> Result<Self, &'static str> {
        let file = File::open(path).map_err(|_| "MCP 설정 파일을 열지 못했습니다.")?;
        let mut bytes = Vec::new();
        file.take(MAX_CONFIG_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| "MCP 설정 파일을 읽지 못했습니다.")?;
        if bytes.len() as u64 > MAX_CONFIG_BYTES {
            return Err("MCP 설정은 64 KiB 이하여야 합니다.");
        }
        Self::from_json(&bytes, profile)
    }

    /// 설정 값은 도구 이름만 포함하며 토큰·계좌·HTTP 접근 권한을 지정하지 않습니다.
    pub fn from_json(bytes: &[u8], profile: &str) -> Result<Self, &'static str> {
        if bytes.len() as u64 > MAX_CONFIG_BYTES {
            return Err("MCP 설정은 64 KiB 이하여야 합니다.");
        }
        let mut config: Profiles =
            serde_json::from_slice(bytes).map_err(|_| "MCP 설정 형식이 올바르지 않습니다.")?;
        if config.version != 1 {
            return Err("지원하지 않는 MCP 설정 버전입니다.");
        }
        let selected = config
            .profiles
            .remove(profile)
            .ok_or("선택한 MCP 프로필이 없습니다.")?;
        for other in config.profiles.into_values() {
            Self::allow_only(other.tools)?;
        }
        Self::allow_only(selected.tools)
    }

    pub(crate) fn allows(&self, name: &str) -> bool {
        self.allowed.contains(name)
    }
}
