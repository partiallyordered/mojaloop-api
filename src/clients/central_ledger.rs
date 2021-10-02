use hyper::client::conn;
use hyper::body::Body;
use fspiox_api::clients::FspiopClient as MojaloopClient;
use crate::central_ledger::{settlement_models, participants};
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
            label: "app.kubernetes.io/name=centralledger-service",
            container_name: "centralledger-service",
            port: k8s::Port::Name("http-api"),
        };

    fn from_sender(sender: conn::SendRequest<Body>) -> Client {
        Client {
            sender
        }
    }
}

pub trait CentralLedgerRequest {
    type Response: serde::de::DeserializeOwned;
}

impl CentralLedgerRequest for participants::GetParticipantLimits {
    type Response = Vec<participants::NewParticipantLimit>;
}

impl CentralLedgerRequest for participants::PutParticipantLimit {
    type Response = NoBody;
}

impl CentralLedgerRequest for participants::PutParticipantAccount {
    type Response = NoBody;
}

impl CentralLedgerRequest for participants::PostHubAccount {
    type Response = NoBody;
}

impl CentralLedgerRequest for participants::PostCallbackUrl {
    type Response = NoBody;
}

impl CentralLedgerRequest for participants::PostParticipantSettlementFunds {
    type Response = NoBody;
}

impl CentralLedgerRequest for participants::GetParticipants {
    type Response = participants::Participants;
}

impl CentralLedgerRequest for participants::GetCallbackUrls {
    type Response = participants::CallbackUrls;
}

impl CentralLedgerRequest for participants::GetDfspAccounts {
    type Response = participants::DfspAccounts;
}

impl CentralLedgerRequest for participants::PostInitialPositionAndLimits {
    type Response = NoBody;
}

impl CentralLedgerRequest for participants::PostParticipant {
    type Response = participants::Participant;
}

impl CentralLedgerRequest for settlement_models::PostSettlementModel {
    type Response = NoBody;
}

impl Client {
    pub async fn send<T: CentralLedgerRequest>(&mut self, msg: T)
        -> fspiox_api::clients::Result<ResponseBody<T::Response>>
    where
        T: CentralLedgerRequest + std::fmt::Debug + Clone,
        http::Request<hyper::Body>: From<T>
    {
        request::<T, T::Response>(&mut self.sender, msg).await
    }
}
