//! market 영역의 REST 호출입니다. 계약과 검증은 domain에서 가져옵니다.

use super::Client;
use crate::runtime::http::Transport;
use kiwoom_core::{ClientError, Page, Secret};
use kiwoom_domain::market::*;

impl<T: Transport> Client<T> {
    /// `usa20520` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20520(
        &self,
        token: &Secret,
        request: &Usa20520Request,
    ) -> Result<Vec<Page<Usa20520Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20520", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20521` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20521(
        &self,
        token: &Secret,
        request: &Usa20521Request,
    ) -> Result<Vec<Page<Usa20521Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20521", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20570` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20570(
        &self,
        token: &Secret,
        request: &Usa20570Request,
    ) -> Result<Vec<Page<Usa20570Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20570", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20571` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20571(
        &self,
        token: &Secret,
        request: &Usa20571Request,
    ) -> Result<Vec<Page<Usa20571Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20571", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20930` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20930(
        &self,
        token: &Secret,
        request: &Usa20930Request,
    ) -> Result<Vec<Page<Usa20930Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20930", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20931` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20931(
        &self,
        token: &Secret,
        request: &Usa20931Request,
    ) -> Result<Vec<Page<Usa20931Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20931", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20932` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20932(
        &self,
        token: &Secret,
        request: &Usa20932Request,
    ) -> Result<Vec<Page<Usa20932Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20932", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20970` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20970(
        &self,
        token: &Secret,
        request: &Usa20970Request,
    ) -> Result<Vec<Page<Usa20970Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20970", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20971` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20971(
        &self,
        token: &Secret,
        request: &Usa20971Request,
    ) -> Result<Vec<Page<Usa20971Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20971", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20972` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20972(
        &self,
        token: &Secret,
        request: &Usa20972Request,
    ) -> Result<Vec<Page<Usa20972Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20972", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa23400` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa23400(
        &self,
        token: &Secret,
        request: &Usa23400Request,
    ) -> Result<Vec<Page<Usa23400Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa23400", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa23401` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa23401(
        &self,
        token: &Secret,
        request: &Usa23401Request,
    ) -> Result<Vec<Page<Usa23401Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa23401", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa23402` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa23402(
        &self,
        token: &Secret,
        request: &Usa23402Request,
    ) -> Result<Vec<Page<Usa23402Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa23402", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24100` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24100(
        &self,
        token: &Secret,
        request: &Usa24100Request,
    ) -> Result<Vec<Page<Usa24100Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24100", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24101` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24101(
        &self,
        token: &Secret,
        request: &Usa24101Request,
    ) -> Result<Vec<Page<Usa24101Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24101", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24140` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24140(
        &self,
        token: &Secret,
        request: &Usa24140Request,
    ) -> Result<Vec<Page<Usa24140Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24140", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24141` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24141(
        &self,
        token: &Secret,
        request: &Usa24141Request,
    ) -> Result<Vec<Page<Usa24141Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24141", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24210` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24210(
        &self,
        token: &Secret,
        request: &Usa24210Request,
    ) -> Result<Vec<Page<Usa24210Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24210", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24211` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24211(
        &self,
        token: &Secret,
        request: &Usa24211Request,
    ) -> Result<Vec<Page<Usa24211Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24211", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24220` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24220(
        &self,
        token: &Secret,
        request: &Usa24220Request,
    ) -> Result<Vec<Page<Usa24220Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24220", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa24221` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa24221(
        &self,
        token: &Secret,
        request: &Usa24221Request,
    ) -> Result<Vec<Page<Usa24221Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa24221", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa26411` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa26411(
        &self,
        token: &Secret,
        request: &Usa26411Request,
    ) -> Result<Vec<Page<Usa26411Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa26411", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa26412` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa26412(
        &self,
        token: &Secret,
        request: &Usa26412Request,
    ) -> Result<Vec<Page<Usa26412Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa26412", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa26413` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa26413(
        &self,
        token: &Secret,
        request: &Usa26413Request,
    ) -> Result<Vec<Page<Usa26413Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa26413", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa26414` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa26414(
        &self,
        token: &Secret,
        request: &Usa26414Request,
    ) -> Result<Vec<Page<Usa26414Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa26414", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa26410` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa26410(
        &self,
        token: &Secret,
        request: &Usa26410Request,
    ) -> Result<Vec<Page<Usa26410Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa26410", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka00198` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka00198(
        &self,
        token: &Secret,
        request: &Ka00198Request,
    ) -> Result<Vec<Page<Ka00198Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka00198", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10013` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10013(
        &self,
        token: &Secret,
        request: &Ka10013Request,
    ) -> Result<Vec<Page<Ka10013Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10013", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10015` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10015(
        &self,
        token: &Secret,
        request: &Ka10015Request,
    ) -> Result<Vec<Page<Ka10015Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10015", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10055` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10055(
        &self,
        token: &Secret,
        request: &Ka10055Request,
    ) -> Result<Vec<Page<Ka10055Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10055", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90003` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90003(
        &self,
        token: &Secret,
        request: &Ka90003Request,
    ) -> Result<Vec<Page<Ka90003Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90003", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90004` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90004(
        &self,
        token: &Secret,
        request: &Ka90004Request,
    ) -> Result<Vec<Page<Ka90004Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90004", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt20016` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn kt20016(
        &self,
        token: &Secret,
        request: &Kt20016Request,
    ) -> Result<Vec<Page<Kt20016Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt20016", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `kt20017` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn kt20017(
        &self,
        token: &Secret,
        request: &Kt20017Request,
    ) -> Result<Vec<Page<Kt20017Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "kt20017", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10011` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10011(
        &self,
        token: &Secret,
        request: &Ka10011Request,
    ) -> Result<Vec<Page<Ka10011Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10011", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10044` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10044(
        &self,
        token: &Secret,
        request: &Ka10044Request,
    ) -> Result<Vec<Page<Ka10044Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10044", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10045` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10045(
        &self,
        token: &Secret,
        request: &Ka10045Request,
    ) -> Result<Vec<Page<Ka10045Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10045", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10078` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10078(
        &self,
        token: &Secret,
        request: &Ka10078Request,
    ) -> Result<Vec<Page<Ka10078Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10078", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10086` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10086(
        &self,
        token: &Secret,
        request: &Ka10086Request,
    ) -> Result<Vec<Page<Ka10086Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10086", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10063` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10063(
        &self,
        token: &Secret,
        request: &Ka10063Request,
    ) -> Result<Vec<Page<Ka10063Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10063", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10066` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10066(
        &self,
        token: &Secret,
        request: &Ka10066Request,
    ) -> Result<Vec<Page<Ka10066Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10066", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka50010` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka50010(
        &self,
        token: &Secret,
        request: &Ka50010Request,
    ) -> Result<Vec<Page<Ka50010Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50010", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka50012` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka50012(
        &self,
        token: &Secret,
        request: &Ka50012Request,
    ) -> Result<Vec<Page<Ka50012Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50012", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka50087` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka50087(
        &self,
        token: &Secret,
        request: &Ka50087Request,
    ) -> Result<Vec<Page<Ka50087Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50087", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka50100` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka50100(
        &self,
        token: &Secret,
        request: &Ka50100Request,
    ) -> Result<Vec<Page<Ka50100Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50100", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka50101` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka50101(
        &self,
        token: &Secret,
        request: &Ka50101Request,
    ) -> Result<Vec<Page<Ka50101Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50101", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90005` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90005(
        &self,
        token: &Secret,
        request: &Ka90005Request,
    ) -> Result<Vec<Page<Ka90005Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90005", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90010` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90010(
        &self,
        token: &Secret,
        request: &Ka90010Request,
    ) -> Result<Vec<Page<Ka90010Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90010", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90006` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90006(
        &self,
        token: &Secret,
        request: &Ka90006Request,
    ) -> Result<Vec<Page<Ka90006Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90006", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90007` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90007(
        &self,
        token: &Secret,
        request: &Ka90007Request,
    ) -> Result<Vec<Page<Ka90007Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90007", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90013` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90013(
        &self,
        token: &Secret,
        request: &Ka90013Request,
    ) -> Result<Vec<Page<Ka90013Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90013", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10016` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10016(
        &self,
        token: &Secret,
        request: &Ka10016Request,
    ) -> Result<Vec<Page<Ka10016Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10016", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10017` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10017(
        &self,
        token: &Secret,
        request: &Ka10017Request,
    ) -> Result<Vec<Page<Ka10017Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10017", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10018` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10018(
        &self,
        token: &Secret,
        request: &Ka10018Request,
    ) -> Result<Vec<Page<Ka10018Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10018", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10019` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10019(
        &self,
        token: &Secret,
        request: &Ka10019Request,
    ) -> Result<Vec<Page<Ka10019Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10019", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10024` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10024(
        &self,
        token: &Secret,
        request: &Ka10024Request,
    ) -> Result<Vec<Page<Ka10024Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10024", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10025` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10025(
        &self,
        token: &Secret,
        request: &Ka10025Request,
    ) -> Result<Vec<Page<Ka10025Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10025", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10026` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10026(
        &self,
        token: &Secret,
        request: &Ka10026Request,
    ) -> Result<Vec<Page<Ka10026Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10026", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10028` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10028(
        &self,
        token: &Secret,
        request: &Ka10028Request,
    ) -> Result<Vec<Page<Ka10028Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10028", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10043` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10043(
        &self,
        token: &Secret,
        request: &Ka10043Request,
    ) -> Result<Vec<Page<Ka10043Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10043", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10052` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10052(
        &self,
        token: &Secret,
        request: &Ka10052Request,
    ) -> Result<Vec<Page<Ka10052Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10052", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10054` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10054(
        &self,
        token: &Secret,
        request: &Ka10054Request,
    ) -> Result<Vec<Page<Ka10054Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10054", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10058` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10058(
        &self,
        token: &Secret,
        request: &Ka10058Request,
    ) -> Result<Vec<Page<Ka10058Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10058", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10059` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10059(
        &self,
        token: &Secret,
        request: &Ka10059Request,
    ) -> Result<Vec<Page<Ka10059Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10059", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10061` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10061(
        &self,
        token: &Secret,
        request: &Ka10061Request,
    ) -> Result<Vec<Page<Ka10061Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10061", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10095` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10095(
        &self,
        token: &Secret,
        request: &Ka10095Request,
    ) -> Result<Vec<Page<Ka10095Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10095", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10007` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10007(
        &self,
        token: &Secret,
        request: &Ka10007Request,
    ) -> Result<Vec<Page<Ka10007Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10007", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10046` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10046(
        &self,
        token: &Secret,
        request: &Ka10046Request,
    ) -> Result<Vec<Page<Ka10046Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10046", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10047` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10047(
        &self,
        token: &Secret,
        request: &Ka10047Request,
    ) -> Result<Vec<Page<Ka10047Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10047", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka90008` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka90008(
        &self,
        token: &Secret,
        request: &Ka90008Request,
    ) -> Result<Vec<Page<Ka90008Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka90008", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10001` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10001(
        &self,
        token: &Secret,
        request: &Ka10001Request,
    ) -> Result<Vec<Page<Ka10001Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10001", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10002` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10002(
        &self,
        token: &Secret,
        request: &Ka10002Request,
    ) -> Result<Vec<Page<Ka10002Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10002", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10003` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10003(
        &self,
        token: &Secret,
        request: &Ka10003Request,
    ) -> Result<Vec<Page<Ka10003Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10003", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10084` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10084(
        &self,
        token: &Secret,
        request: &Ka10084Request,
    ) -> Result<Vec<Page<Ka10084Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10084", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10004` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10004(
        &self,
        token: &Secret,
        request: &Ka10004Request,
    ) -> Result<Vec<Page<Ka10004Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10004", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10005` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10005(
        &self,
        token: &Secret,
        request: &Ka10005Request,
    ) -> Result<Vec<Page<Ka10005Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10005", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10006` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10006(
        &self,
        token: &Secret,
        request: &Ka10006Request,
    ) -> Result<Vec<Page<Ka10006Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10006", "/api/dostk/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10100` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10100(
        &self,
        token: &Secret,
        request: &Ka10100Request,
    ) -> Result<Vec<Page<Ka10100Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10100", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10099` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10099(
        &self,
        token: &Secret,
        request: &Ka10099Request,
    ) -> Result<Vec<Page<Ka10099Response>>, ClientError> {
        self.read_pages(token, request, "ka10099", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10102` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10102(
        &self,
        token: &Secret,
        request: &Ka10102Request,
    ) -> Result<Vec<Page<Ka10102Response>>, ClientError> {
        self.read_pages(token, request, "ka10102", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10103` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10103(
        &self,
        token: &Secret,
        request: &Ka10103Request,
    ) -> Result<Vec<Page<Ka10103Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10103", "/api/dostk/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10098` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10098(
        &self,
        token: &Secret,
        request: &Usa10098Request,
    ) -> Result<Vec<Page<Usa10098Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa10098", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10099` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10099(
        &self,
        token: &Secret,
        request: &Usa10099Request,
    ) -> Result<Vec<Page<Usa10099Response>>, ClientError> {
        self.read_pages(token, request, "usa10099", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10100` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10100(
        &self,
        token: &Secret,
        request: &Usa10100Request,
    ) -> Result<Vec<Page<Usa10100Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa10100", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10101` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10101(
        &self,
        token: &Secret,
        request: &Usa10101Request,
    ) -> Result<Vec<Page<Usa10101Response>>, ClientError> {
        self.read_pages(token, request, "usa10101", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10102` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10102(
        &self,
        token: &Secret,
        request: &Usa10102Request,
    ) -> Result<Vec<Page<Usa10102Response>>, ClientError> {
        self.read_pages(token, request, "usa10102", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10103` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10103(
        &self,
        token: &Secret,
        request: &Usa10103Request,
    ) -> Result<Vec<Page<Usa10103Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa10103", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10104` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10104(
        &self,
        token: &Secret,
        request: &Usa10104Request,
    ) -> Result<Vec<Page<Usa10104Response>>, ClientError> {
        self.read_pages(token, request, "usa10104", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa10105` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa10105(
        &self,
        token: &Secret,
        request: &Usa10105Request,
    ) -> Result<Vec<Page<Usa10105Response>>, ClientError> {
        self.read_pages(token, request, "usa10105", "/api/us/stkinfo")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20100` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20100(
        &self,
        token: &Secret,
        request: &Usa20100Request,
    ) -> Result<Vec<Page<Usa20100Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20100", "/api/us/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20101` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20101(
        &self,
        token: &Secret,
        request: &Usa20101Request,
    ) -> Result<Vec<Page<Usa20101Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20101", "/api/us/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20150` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20150(
        &self,
        token: &Secret,
        request: &Usa20150Request,
    ) -> Result<Vec<Page<Usa20150Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20150", "/api/us/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20151` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20151(
        &self,
        token: &Secret,
        request: &Usa20151Request,
    ) -> Result<Vec<Page<Usa20151Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20151", "/api/us/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `usa20590` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn usa20590(
        &self,
        token: &Secret,
        request: &Usa20590Request,
    ) -> Result<Vec<Page<Usa20590Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa20590", "/api/us/mrkcond")
            .await
    }
}

impl<T: Transport> Client<T> {
    /// `ka10101` 조회의 모든 연속 페이지를 원본 응답과 함께 반환한다.
    pub async fn ka10101(
        &self,
        token: &Secret,
        request: &Ka10101Request,
    ) -> Result<Vec<Page<Ka10101Response>>, ClientError> {
        self.read_pages(token, request, "ka10101", "/api/dostk/stkinfo")
            .await
    }
}
