use hyper::client::conn;
use hyper::body::Body;
use fspiox_api::clients::FspiopClient as MojaloopClient;
use crate::settlement::{settlement, settlement_windows};
use fspiox_api::clients::{request, NoBody};
#[cfg(feature = "clients-kube")]
use fspiox_api::clients::k8s;

#[derive(Debug)]
pub struct Client {
    sender: conn::SendRequest<Body>,
}

#[derive(Debug)]
pub enum Request {
    PostSettlement(settlement::PostSettlement),
    GetSettlements(settlement::GetSettlements),
    CloseSettlementWindow(settlement_windows::CloseSettlementWindow),
    GetSettlementWindow(settlement_windows::GetSettlementWindow),
    GetSettlementWindows(settlement_windows::GetSettlementWindows),
}

impl From<settlement::PostSettlement> for Request {
    fn from(i: settlement::PostSettlement) -> Request {
        Request::PostSettlement(i)
    }
}

impl From<settlement::GetSettlements> for Request {
    fn from(i: settlement::GetSettlements) -> Request {
        Request::GetSettlements(i)
    }
}

impl From<settlement_windows::CloseSettlementWindow> for Request {
    fn from(i: settlement_windows::CloseSettlementWindow) -> Request {
        Request::CloseSettlementWindow(i)
    }
}

impl From<settlement_windows::GetSettlementWindow> for Request {
    fn from(i: settlement_windows::GetSettlementWindow) -> Request {
        Request::GetSettlementWindow(i)
    }
}

impl From<settlement_windows::GetSettlementWindows> for Request {
    fn from(i: settlement_windows::GetSettlementWindows) -> Request {
        Request::GetSettlementWindows(i)
    }
}

impl From<Request> for http::Request<hyper::Body> {
    fn from(item: Request) -> http::Request<hyper::Body> {
        match item {
            Request::PostSettlement(i) => i.into(),
            Request::GetSettlements(i) => i.into(),
            Request::CloseSettlementWindow(i) => i.into(),
            Request::GetSettlementWindow(i) => i.into(),
            Request::GetSettlementWindows(i) => i.into(),
        }
    }
}

impl MojaloopClient for Client {
    #[cfg(feature = "clients-kube")]
    const K8S_PARAMS: k8s::KubernetesParams =
        k8s::KubernetesParams {
            label: "app.kubernetes.io/name=centralsettlement-service",
            container_name: "centralsettlement-service",
            port: k8s::Port::Number(3007),
        };

    fn from_sender(sender: conn::SendRequest<Body>) -> Client {
        Client {
            sender
        }
    }
}

#[derive(Debug)]
pub enum Response {
    PostSettlement(settlement::Settlement),
    GetSettlements(settlement::Settlements),
    GetSettlementWindow(settlement_windows::SettlementWindow),
    GetSettlementWindows(settlement_windows::SettlementWindows),
    CloseSettlementWindow(NoBody),
}

impl Client {
    pub async fn send(&mut self, msg: Request) -> fspiox_api::clients::Result<Response> {
        use crate::settlement::settlement::*;
        use crate::settlement::settlement_windows::*;
        Ok(
            match msg {
                Request::PostSettlement(m) => Response::PostSettlement(
                    request::<PostSettlement, Settlement>(&mut self.sender, m).await?),
                Request::GetSettlements(m) => Response::GetSettlements(
                    request::<GetSettlements, Settlements>(&mut self.sender, m).await?),
                Request::CloseSettlementWindow(m) => Response::CloseSettlementWindow(
                    request::<CloseSettlementWindow, NoBody>(&mut self.sender, m).await?),
                Request::GetSettlementWindow(m) => Response::GetSettlementWindow(
                    request::<GetSettlementWindow, SettlementWindow>(&mut self.sender, m).await?),
                Request::GetSettlementWindows(m) => Response::GetSettlementWindows(
                    request::<GetSettlementWindows, SettlementWindows>(&mut self.sender, m).await?),
            }
        )
    }
}
