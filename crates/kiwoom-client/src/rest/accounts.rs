//! accounts 영역의 REST 호출입니다. 계약과 검증은 domain에서 가져옵니다.

use super::Client;
use crate::runtime::http::Transport;
use kiwoom_core::{ClientError, Page, Secret};
use kiwoom_domain::accounts::*;

impl<T: Transport> Client<T> {
    /// 국내주식 거래 이력을 페이지 상한 안에서 수집하고 연속 조회 증거를 검증합니다.
    pub async fn kt00015(
        &self,
        token: &Secret,
        r: &Kt00015Request,
    ) -> Result<Vec<Page<Kt00015Response>>, ClientError> {
        r.validate()?;
        self.read_pages(token, r, "kt00015", "/api/dostk/acnt")
            .await
    }

    /// 국내주식 거래 이력 한 페이지와 원본 연속 조회 헤더를 반환합니다.
    /// 전체 조회가 끝났는지 판단하려면 호출자가 수집한 페이지 체인을 검증해야 합니다.
    pub async fn kt00015_page(
        &self,
        token: &Secret,
        request: &Kt00015Request,
        continuation: Option<&str>,
    ) -> Result<Page<Kt00015Response>, ClientError> {
        request.validate()?;
        self.read_page(token, request, "kt00015", "/api/dostk/acnt", continuation)
            .await
    }
}

impl<T: Transport> Client<T> {
    /// 미국주식 거래 이력을 페이지 상한 안에서 수집하고 연속 조회 증거를 검증합니다.
    pub async fn ust21100(
        &self,
        token: &Secret,
        r: &Ust21100Request,
    ) -> Result<Vec<Page<Ust21100Response>>, ClientError> {
        r.validate()?;
        self.read_pages(token, r, "ust21100", "/api/us/acnt").await
    }

    /// 문서화된 typed 필터로 미국주식 거래 이력 한 페이지를 조회합니다.
    /// 전체 조회가 끝났는지 판단하려면 호출자가 수집한 페이지 체인을 검증해야 합니다.
    pub async fn ust21100_page(
        &self,
        token: &Secret,
        request: &Ust21100Request,
        continuation: Option<&str>,
    ) -> Result<Page<Ust21100Response>, ClientError> {
        request.validate()?;
        self.read_page(token, request, "ust21100", "/api/us/acnt", continuation)
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka00001` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka00001(
        &self,
        token: &Secret,
        request: &Ka00001Request,
    ) -> Result<Vec<Page<Ka00001Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka00001", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka01690` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka01690(
        &self,
        token: &Secret,
        request: &Ka01690Request,
    ) -> Result<Vec<Page<Ka01690Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka01690", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10072` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10072(
        &self,
        token: &Secret,
        request: &Ka10072Request,
    ) -> Result<Vec<Page<Ka10072Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10072", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10073` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10073(
        &self,
        token: &Secret,
        request: &Ka10073Request,
    ) -> Result<Vec<Page<Ka10073Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10073", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10074` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10074(
        &self,
        token: &Secret,
        request: &Ka10074Request,
    ) -> Result<Vec<Page<Ka10074Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10074", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10075` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10075(
        &self,
        token: &Secret,
        request: &Ka10075Request,
    ) -> Result<Vec<Page<Ka10075Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10075", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10076` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10076(
        &self,
        token: &Secret,
        request: &Ka10076Request,
    ) -> Result<Vec<Page<Ka10076Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10076", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10077` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10077(
        &self,
        token: &Secret,
        request: &Ka10077Request,
    ) -> Result<Vec<Page<Ka10077Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10077", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10085` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10085(
        &self,
        token: &Secret,
        request: &Ka10085Request,
    ) -> Result<Vec<Page<Ka10085Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10085", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10088` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10088(
        &self,
        token: &Secret,
        request: &Ka10088Request,
    ) -> Result<Vec<Page<Ka10088Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10088", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10170` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ka10170(
        &self,
        token: &Secret,
        request: &Ka10170Request,
    ) -> Result<Vec<Page<Ka10170Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10170", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00001` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00001(
        &self,
        token: &Secret,
        request: &Kt00001Request,
    ) -> Result<Vec<Page<Kt00001Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00001", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00002` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00002(
        &self,
        token: &Secret,
        request: &Kt00002Request,
    ) -> Result<Vec<Page<Kt00002Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00002", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00003` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00003(
        &self,
        token: &Secret,
        request: &Kt00003Request,
    ) -> Result<Vec<Page<Kt00003Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00003", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00004` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00004(
        &self,
        token: &Secret,
        request: &Kt00004Request,
    ) -> Result<Vec<Page<Kt00004Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00004", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00005` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00005(
        &self,
        token: &Secret,
        request: &Kt00005Request,
    ) -> Result<Vec<Page<Kt00005Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00005", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00007` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00007(
        &self,
        token: &Secret,
        request: &Kt00007Request,
    ) -> Result<Vec<Page<Kt00007Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00007", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00008` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00008(
        &self,
        token: &Secret,
        request: &Kt00008Request,
    ) -> Result<Vec<Page<Kt00008Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00008", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00009` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00009(
        &self,
        token: &Secret,
        request: &Kt00009Request,
    ) -> Result<Vec<Page<Kt00009Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00009", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00010` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00010(
        &self,
        token: &Secret,
        request: &Kt00010Request,
    ) -> Result<Vec<Page<Kt00010Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00010", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00011` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00011(
        &self,
        token: &Secret,
        request: &Kt00011Request,
    ) -> Result<Vec<Page<Kt00011Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00011", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00012` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00012(
        &self,
        token: &Secret,
        request: &Kt00012Request,
    ) -> Result<Vec<Page<Kt00012Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00012", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00013` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00013(
        &self,
        token: &Secret,
        request: &Kt00013Request,
    ) -> Result<Vec<Page<Kt00013Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00013", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00016` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00016(
        &self,
        token: &Secret,
        request: &Kt00016Request,
    ) -> Result<Vec<Page<Kt00016Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00016", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00017` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00017(
        &self,
        token: &Secret,
        request: &Kt00017Request,
    ) -> Result<Vec<Page<Kt00017Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00017", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt00018` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt00018(
        &self,
        token: &Secret,
        request: &Kt00018Request,
    ) -> Result<Vec<Page<Kt00018Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt00018", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt50020` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt50020(
        &self,
        token: &Secret,
        request: &Kt50020Request,
    ) -> Result<Vec<Page<Kt50020Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt50020", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt50021` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt50021(
        &self,
        token: &Secret,
        request: &Kt50021Request,
    ) -> Result<Vec<Page<Kt50021Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt50021", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt50030` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt50030(
        &self,
        token: &Secret,
        request: &Kt50030Request,
    ) -> Result<Vec<Page<Kt50030Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt50030", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt50031` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt50031(
        &self,
        token: &Secret,
        request: &Kt50031Request,
    ) -> Result<Vec<Page<Kt50031Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt50031", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt50032` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt50032(
        &self,
        token: &Secret,
        request: &Kt50032Request,
    ) -> Result<Vec<Page<Kt50032Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt50032", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt50075` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn kt50075(
        &self,
        token: &Secret,
        request: &Kt50075Request,
    ) -> Result<Vec<Page<Kt50075Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt50075", "/api/dostk/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa21670` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn usa21670(
        &self,
        token: &Secret,
        request: &Usa21670Request,
    ) -> Result<Vec<Page<Usa21670Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa21670", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa21680` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn usa21680(
        &self,
        token: &Secret,
        request: &Usa21680Request,
    ) -> Result<Vec<Page<Usa21680Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa21680", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa21690` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn usa21690(
        &self,
        token: &Secret,
        request: &Usa21690Request,
    ) -> Result<Vec<Page<Usa21690Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa21690", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa21730` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn usa21730(
        &self,
        token: &Secret,
        request: &Usa21730Request,
    ) -> Result<Vec<Page<Usa21730Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa21730", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa21731` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn usa21731(
        &self,
        token: &Secret,
        request: &Usa21731Request,
    ) -> Result<Vec<Page<Usa21731Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa21731", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa21732` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn usa21732(
        &self,
        token: &Secret,
        request: &Usa21732Request,
    ) -> Result<Vec<Page<Usa21732Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa21732", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21050` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21050(
        &self,
        token: &Secret,
        request: &Ust21050Request,
    ) -> Result<Vec<Page<Ust21050Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21050", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21070` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21070(
        &self,
        token: &Secret,
        request: &Ust21070Request,
    ) -> Result<Vec<Page<Ust21070Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21070", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21110` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21110(
        &self,
        token: &Secret,
        request: &Ust21110Request,
    ) -> Result<Vec<Page<Ust21110Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21110", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21111` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21111(
        &self,
        token: &Secret,
        request: &Ust21111Request,
    ) -> Result<Vec<Page<Ust21111Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21111", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21120` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21120(
        &self,
        token: &Secret,
        request: &Ust21120Request,
    ) -> Result<Vec<Page<Ust21120Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21120", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21121` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21121(
        &self,
        token: &Secret,
        request: &Ust21121Request,
    ) -> Result<Vec<Page<Ust21121Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21121", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21131` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21131(
        &self,
        token: &Secret,
        request: &Ust21131Request,
    ) -> Result<Vec<Page<Ust21131Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21131", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21132` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21132(
        &self,
        token: &Secret,
        request: &Ust21132Request,
    ) -> Result<Vec<Page<Ust21132Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21132", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21150` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21150(
        &self,
        token: &Secret,
        request: &Ust21150Request,
    ) -> Result<Vec<Page<Ust21150Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21150", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21160` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21160(
        &self,
        token: &Secret,
        request: &Ust21160Request,
    ) -> Result<Vec<Page<Ust21160Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21160", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21170` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21170(
        &self,
        token: &Secret,
        request: &Ust21170Request,
    ) -> Result<Vec<Page<Ust21170Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21170", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21180` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21180(
        &self,
        token: &Secret,
        request: &Ust21180Request,
    ) -> Result<Vec<Page<Ust21180Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21180", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21510` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21510(
        &self,
        token: &Secret,
        request: &Ust21510Request,
    ) -> Result<Vec<Page<Ust21510Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21510", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21530` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21530(
        &self,
        token: &Secret,
        request: &Ust21530Request,
    ) -> Result<Vec<Page<Ust21530Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21530", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21610` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21610(
        &self,
        token: &Secret,
        request: &Ust21610Request,
    ) -> Result<Vec<Page<Ust21610Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21610", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21620` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21620(
        &self,
        token: &Secret,
        request: &Ust21620Request,
    ) -> Result<Vec<Page<Ust21620Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21620", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21630` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21630(
        &self,
        token: &Secret,
        request: &Ust21630Request,
    ) -> Result<Vec<Page<Ust21630Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21630", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21640` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21640(
        &self,
        token: &Secret,
        request: &Ust21640Request,
    ) -> Result<Vec<Page<Ust21640Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21640", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21650` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21650(
        &self,
        token: &Secret,
        request: &Ust21650Request,
    ) -> Result<Vec<Page<Ust21650Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21650", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21660` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21660(
        &self,
        token: &Secret,
        request: &Ust21660Request,
    ) -> Result<Vec<Page<Ust21660Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21660", "/api/us/acnt")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ust21661` 요청을 검증한 뒤 계좌 조회의 연속 페이지를 가져옵니다.
    pub async fn ust21661(
        &self,
        token: &Secret,
        request: &Ust21661Request,
    ) -> Result<Vec<Page<Ust21661Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ust21661", "/api/us/acnt")
            .await
    }
}
