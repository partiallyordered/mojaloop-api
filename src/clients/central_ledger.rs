use hyper::client::conn;
use hyper::body::Body;
use fspiox_api::clients::FspiopClient as MojaloopClient;
use crate::central_ledger::participants;
use fspiox_api::clients::{request, NoBody};

pub struct Client {
    sender: conn::SendRequest<Body>,
}

pub enum Request {
    GetParticipantLimits(participants::GetParticipantLimits),
    PutParticipantLimit(participants::PutParticipantLimit),
    PutParticipantAccount(participants::PutParticipantAccount),
    PostHubAccount(participants::PostHubAccount),
    PostCallbackUrl(participants::PostCallbackUrl),
    PostParticipantSettlementFunds(participants::PostParticipantSettlementFunds),
    GetParticipants(participants::GetParticipants),
    GetCallbackUrls(participants::GetCallbackUrls),
    GetDfspAccounts(participants::GetDfspAccounts),
    PostInitialPositionAndLimits(participants::PostInitialPositionAndLimits),
    PostParticipant(participants::PostParticipant),
}

impl From<Request> for http::Request<hyper::Body> {
    fn from(item: Request) -> http::Request<hyper::Body> {
        match item {
            Request::GetParticipantLimits(i) => i.into(),
            Request::PutParticipantLimit(i) => i.into(),
            Request::PutParticipantAccount(i) => i.into(),
            Request::PostHubAccount(i) => i.into(),
            Request::PostCallbackUrl(i) => i.into(),
            Request::PostParticipantSettlementFunds(i) => i.into(),
            Request::GetParticipants(i) => i.into(),
            Request::GetCallbackUrls(i) => i.into(),
            Request::GetDfspAccounts(i) => i.into(),
            Request::PostInitialPositionAndLimits(i) => i.into(),
            Request::PostParticipant(i) => i.into(),
        }
    }
}

impl MojaloopClient for Client {
    #[cfg(feature = "clients-kube")]
    const K8S_PARAMS: k8s::KubernetesParams =
        k8s::KubernetesParams {
            label: "app.kubernetes.io/name=quoting-service",
            container_name: "quoting-service",
            port: k8s::Port::Name("http-api"),
        };

    fn from_sender(sender: conn::SendRequest<Body>) -> Client {
        Client {
            sender
        }
    }
}

pub enum Response {
    GetParticipantLimits(Vec<participants::NewParticipantLimit>),
    PutParticipantLimit(NoBody),
    PutParticipantAccount(NoBody),
    PostHubAccount(NoBody),
    PostCallbackUrl(NoBody),
    PostParticipantSettlementFunds(NoBody),
    GetParticipants(participants::Participants),
    GetCallbackUrls(participants::CallbackUrls),
    GetDfspAccounts(participants::DfspAccounts),
    PostInitialPositionAndLimits(NoBody),
    PostParticipant(participants::Participant),
}

impl Client {
    pub async fn send(&mut self, msg: Request) -> fspiox_api::clients::Result<Response> {
        use crate::central_ledger::participants::*;
        Ok(
            match msg {
                Request::GetParticipantLimits(m) => Response::GetParticipantLimits(
                    request::<GetParticipantLimits, Vec<NewParticipantLimit>>(&mut self.sender, m).await?),
                Request::PutParticipantLimit(m) => Response::PutParticipantLimit(
                    request::<PutParticipantLimit, NoBody>(&mut self.sender, m).await?),
                Request::PutParticipantAccount(m) => Response::PutParticipantAccount(
                    request::<PutParticipantAccount, NoBody>(&mut self.sender, m).await?),
                Request::PostHubAccount(m) => Response::PostHubAccount(
                    request::<PostHubAccount, NoBody>(&mut self.sender, m).await?),
                Request::PostCallbackUrl(m) => Response::PostCallbackUrl(
                    request::<PostCallbackUrl, NoBody>(&mut self.sender, m).await?),
                Request::PostParticipantSettlementFunds(m) => Response::PostParticipantSettlementFunds(
                    request::<PostParticipantSettlementFunds, NoBody>(&mut self.sender, m).await?),
                Request::GetParticipants(m) => Response::GetParticipants(
                    request::<GetParticipants, Participants>(&mut self.sender, m).await?),
                Request::GetCallbackUrls(m) => Response::GetCallbackUrls(
                    request::<GetCallbackUrls, CallbackUrls>(&mut self.sender, m).await?),
                Request::GetDfspAccounts(m) => Response::GetDfspAccounts(
                    request::<GetDfspAccounts, DfspAccounts>(&mut self.sender, m).await?),
                Request::PostInitialPositionAndLimits(m) => Response::PostInitialPositionAndLimits(
                    request::<PostInitialPositionAndLimits, NoBody>(&mut self.sender, m).await?),
                Request::PostParticipant(m) => Response::PostParticipant(
                    request::<PostParticipant, Participant>(&mut self.sender, m).await?),
            }
        )
    }
}
