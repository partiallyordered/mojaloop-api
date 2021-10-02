use hyper::client::conn;
use hyper::body::Body;
use fspiox_api::clients::FspiopClient as MojaloopClient;
use crate::settlement::{settlement, settlement_windows};
use fspiox_api::clients::{request, NoBody, ResponseBody};
#[cfg(feature = "clients-kube")]
use fspiox_api::clients::k8s;

#[derive(Debug)]
pub struct Client {
    sender: conn::SendRequest<Body>,
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

pub trait SettlementRequest {
    type Response: serde::de::DeserializeOwned;
}

impl SettlementRequest for settlement::PostSettlement {
    type Response = settlement::Settlement;
}

impl SettlementRequest for settlement::GetSettlements {
    type Response = settlement::Settlements;
}

impl SettlementRequest for settlement_windows::GetSettlementWindow {
    type Response = settlement_windows::SettlementWindow;
}

impl SettlementRequest for settlement_windows::GetSettlementWindows {
    type Response = settlement_windows::SettlementWindows;
}

impl SettlementRequest for settlement_windows::CloseSettlementWindow {
    type Response = NoBody;
}

impl Client {
    pub async fn send<T: SettlementRequest>(&mut self, msg: T)
        -> fspiox_api::clients::Result<ResponseBody<T::Response>>
    where
        T: SettlementRequest + std::fmt::Debug + Clone,
        http::Request<hyper::Body>: From<T>
    {
        request::<T, T::Response>(&mut self.sender, msg).await
    }
}
