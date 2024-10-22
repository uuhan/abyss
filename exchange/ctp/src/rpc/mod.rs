#![allow(missing_docs)]
mod rpc {
    tonic::include_proto!("ctp");
}

pub use self::rpc::ctp_client::*;
pub use self::rpc::ctp_server::*;
pub use self::rpc::*;

use tonic::{Request, Response, Status};

use crate::api::*;
use crate::*;

#[derive(Debug)]
pub struct CTPService<'a> {
    trader: CTPTrader<'a>,
}

impl<'a> CTPService<'a> {
    pub fn new() -> Self {
        let trader = CTPTrader::default();
        trader.init_by_env(empty!(_)).unwrap();
        CTPService { trader }
    }

    pub fn req_qry_instrument(&self, name: &str) -> CTPResult<Option<CThostFtdcInstrumentField>> {
        self.trader.req_qry_instrument_sync(name)
    }

    pub fn req_qry_instruments(&self, name: &str) -> CTPResult<Vec<CThostFtdcInstrumentField>> {
        self.trader.req_qry_instruments_sync(name)
    }
}

#[tonic::async_trait]
impl Ctp for CTPService<'static> {
    async fn version(
        &self,
        request: Request<VersionRequest>,
    ) -> Result<Response<VersionReply>, Status> {
        let reply = VersionReply {
            version: version().to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn req_qry_instrument(
        &self,
        request: Request<InstrumentRequest>,
    ) -> Result<Response<InstrumentReply>, Status> {
        let v = self
            .req_qry_instrument(&request.into_inner().name)
            .unwrap()
            .unwrap();
        let reply = InstrumentReply {
            reply: serde_json::to_string(&v).unwrap(),
        };

        Ok(Response::new(reply))
    }
}
