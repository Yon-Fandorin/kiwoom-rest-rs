//! charts 영역의 REST 호출입니다. 계약과 검증은 domain에서 가져옵니다.

use super::Client;
use crate::runtime::http::Transport;
use kiwoom_core::{ClientError, Page, Secret};
use kiwoom_domain::charts::*;

impl<T: Transport> Client<T> {
    pub async fn ka10010(
        &self,
        token: &Secret,
        request: &Ka10010Request,
    ) -> Result<Vec<Page<Ka10010Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10010", "/api/dostk/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10051(
        &self,
        token: &Secret,
        request: &Ka10051Request,
    ) -> Result<Vec<Page<Ka10051Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10051", "/api/dostk/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20001(
        &self,
        token: &Secret,
        request: &Ka20001Request,
    ) -> Result<Vec<Page<Ka20001Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20001", "/api/dostk/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20002(
        &self,
        token: &Secret,
        request: &Ka20002Request,
    ) -> Result<Vec<Page<Ka20002Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20002", "/api/dostk/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20003(
        &self,
        token: &Secret,
        request: &Ka20003Request,
    ) -> Result<Vec<Page<Ka20003Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20003", "/api/dostk/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20009(
        &self,
        token: &Secret,
        request: &Ka20009Request,
    ) -> Result<Vec<Page<Ka20009Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20009", "/api/dostk/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10060(
        &self,
        token: &Secret,
        request: &Ka10060Request,
    ) -> Result<Vec<Page<Ka10060Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10060", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10064(
        &self,
        token: &Secret,
        request: &Ka10064Request,
    ) -> Result<Vec<Page<Ka10064Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10064", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10079(
        &self,
        token: &Secret,
        request: &Ka10079Request,
    ) -> Result<Vec<Page<Ka10079Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10079", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka11079(
        &self,
        token: &Secret,
        request: &Ka11079Request,
    ) -> Result<Vec<Page<Ka11079Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka11079", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10080(
        &self,
        token: &Secret,
        request: &Ka10080Request,
    ) -> Result<Vec<Page<Ka10080Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10080", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka11080(
        &self,
        token: &Secret,
        request: &Ka11080Request,
    ) -> Result<Vec<Page<Ka11080Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka11080", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10081(
        &self,
        token: &Secret,
        request: &Ka10081Request,
    ) -> Result<Vec<Page<Ka10081Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10081", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10082(
        &self,
        token: &Secret,
        request: &Ka10082Request,
    ) -> Result<Vec<Page<Ka10082Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10082", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10083(
        &self,
        token: &Secret,
        request: &Ka10083Request,
    ) -> Result<Vec<Page<Ka10083Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10083", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka11081(
        &self,
        token: &Secret,
        request: &Ka11081Request,
    ) -> Result<Vec<Page<Ka11081Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka11081", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka11082(
        &self,
        token: &Secret,
        request: &Ka11082Request,
    ) -> Result<Vec<Page<Ka11082Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka11082", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka11083(
        &self,
        token: &Secret,
        request: &Ka11083Request,
    ) -> Result<Vec<Page<Ka11083Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka11083", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka10094(
        &self,
        token: &Secret,
        request: &Ka10094Request,
    ) -> Result<Vec<Page<Ka10094Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka10094", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka11094(
        &self,
        token: &Secret,
        request: &Ka11094Request,
    ) -> Result<Vec<Page<Ka11094Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka11094", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20004(
        &self,
        token: &Secret,
        request: &Ka20004Request,
    ) -> Result<Vec<Page<Ka20004Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20004", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20005(
        &self,
        token: &Secret,
        request: &Ka20005Request,
    ) -> Result<Vec<Page<Ka20005Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20005", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20006(
        &self,
        token: &Secret,
        request: &Ka20006Request,
    ) -> Result<Vec<Page<Ka20006Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20006", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20007(
        &self,
        token: &Secret,
        request: &Ka20007Request,
    ) -> Result<Vec<Page<Ka20007Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20007", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20008(
        &self,
        token: &Secret,
        request: &Ka20008Request,
    ) -> Result<Vec<Page<Ka20008Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20008", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka20019(
        &self,
        token: &Secret,
        request: &Ka20019Request,
    ) -> Result<Vec<Page<Ka20019Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka20019", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka50079(
        &self,
        token: &Secret,
        request: &Ka50079Request,
    ) -> Result<Vec<Page<Ka50079Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50079", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka50080(
        &self,
        token: &Secret,
        request: &Ka50080Request,
    ) -> Result<Vec<Page<Ka50080Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50080", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka50081(
        &self,
        token: &Secret,
        request: &Ka50081Request,
    ) -> Result<Vec<Page<Ka50081Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50081", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka50082(
        &self,
        token: &Secret,
        request: &Ka50082Request,
    ) -> Result<Vec<Page<Ka50082Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50082", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka50083(
        &self,
        token: &Secret,
        request: &Ka50083Request,
    ) -> Result<Vec<Page<Ka50083Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50083", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka50091(
        &self,
        token: &Secret,
        request: &Ka50091Request,
    ) -> Result<Vec<Page<Ka50091Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50091", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn ka50092(
        &self,
        token: &Secret,
        request: &Ka50092Request,
    ) -> Result<Vec<Page<Ka50092Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "ka50092", "/api/dostk/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa23000(
        &self,
        token: &Secret,
        request: &Usa23000Request,
    ) -> Result<Vec<Page<Usa23000Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa23000", "/api/us/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa23100(
        &self,
        token: &Secret,
        request: &Usa23100Request,
    ) -> Result<Vec<Page<Usa23100Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa23100", "/api/us/sect")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa06010(
        &self,
        token: &Secret,
        request: &Usa06010Request,
    ) -> Result<Vec<Page<Usa06010Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa06010", "/api/us/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa06011(
        &self,
        token: &Secret,
        request: &Usa06011Request,
    ) -> Result<Vec<Page<Usa06011Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa06011", "/api/us/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa06012(
        &self,
        token: &Secret,
        request: &Usa06012Request,
    ) -> Result<Vec<Page<Usa06012Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa06012", "/api/us/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa06013(
        &self,
        token: &Secret,
        request: &Usa06013Request,
    ) -> Result<Vec<Page<Usa06013Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa06013", "/api/us/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa06014(
        &self,
        token: &Secret,
        request: &Usa06014Request,
    ) -> Result<Vec<Page<Usa06014Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa06014", "/api/us/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa06015(
        &self,
        token: &Secret,
        request: &Usa06015Request,
    ) -> Result<Vec<Page<Usa06015Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa06015", "/api/us/chart")
            .await
    }
}

impl<T: Transport> Client<T> {
    pub async fn usa06016(
        &self,
        token: &Secret,
        request: &Usa06016Request,
    ) -> Result<Vec<Page<Usa06016Response>>, ClientError> {
        request.validate()?;
        self.read_pages(token, request, "usa06016", "/api/us/chart")
            .await
    }
}
