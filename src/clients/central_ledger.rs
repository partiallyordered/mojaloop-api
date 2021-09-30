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

#[derive(Debug)]
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
    PostSettlementModel(settlement_models::PostSettlementModel),
}

// TODO: should these impls be a macro?
impl From<participants::GetParticipantLimits> for Request {
    fn from(i: participants::GetParticipantLimits) -> Request {
        Request::GetParticipantLimits(i)
    }
}

impl From<participants::PutParticipantLimit> for Request {
    fn from(i: participants::PutParticipantLimit) -> Request {
        Request::PutParticipantLimit(i)
    }
}

impl From<participants::PutParticipantAccount> for Request {
    fn from(i: participants::PutParticipantAccount) -> Request {
        Request::PutParticipantAccount(i)
    }
}

impl From<participants::PostHubAccount> for Request {
    fn from(i: participants::PostHubAccount) -> Request {
        Request::PostHubAccount(i)
    }
}

impl From<participants::PostCallbackUrl> for Request {
    fn from(i: participants::PostCallbackUrl) -> Request {
        Request::PostCallbackUrl(i)
    }
}

impl From<participants::PostParticipantSettlementFunds> for Request {
    fn from(i: participants::PostParticipantSettlementFunds) -> Request {
        Request::PostParticipantSettlementFunds(i)
    }
}

impl From<participants::GetParticipants> for Request {
    fn from(i: participants::GetParticipants) -> Request {
        Request::GetParticipants(i)
    }
}

impl From<participants::GetCallbackUrls> for Request {
    fn from(i: participants::GetCallbackUrls) -> Request {
        Request::GetCallbackUrls(i)
    }
}

impl From<participants::GetDfspAccounts> for Request {
    fn from(i: participants::GetDfspAccounts) -> Request {
        Request::GetDfspAccounts(i)
    }
}

impl From<participants::PostInitialPositionAndLimits> for Request {
    fn from(i: participants::PostInitialPositionAndLimits) -> Request {
        Request::PostInitialPositionAndLimits(i)
    }
}

impl From<participants::PostParticipant> for Request {
    fn from(i: participants::PostParticipant) -> Request {
        Request::PostParticipant(i)
    }
}

impl From<settlement_models::PostSettlementModel> for Request {
    fn from(i: settlement_models::PostSettlementModel) -> Request {
        Request::PostSettlementModel(i)
    }
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
            Request::PostSettlementModel(i) => i.into(),
        }
    }
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

#[derive(Debug)]
pub enum Response {
    GetParticipantLimits(ResponseBody<Vec<participants::NewParticipantLimit>>),
    PutParticipantLimit(ResponseBody<NoBody>),
    PutParticipantAccount(ResponseBody<NoBody>),
    PostHubAccount(ResponseBody<NoBody>),
    PostCallbackUrl(ResponseBody<NoBody>),
    PostParticipantSettlementFunds(ResponseBody<NoBody>),
    GetParticipants(ResponseBody<participants::Participants>),
    GetCallbackUrls(ResponseBody<participants::CallbackUrls>),
    GetDfspAccounts(ResponseBody<participants::DfspAccounts>),
    PostInitialPositionAndLimits(ResponseBody<NoBody>),
    PostParticipant(ResponseBody<participants::Participant>),
    PostSettlementModel(ResponseBody<NoBody>),
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
                Request::PostSettlementModel(m) => Response::PostSettlementModel(
                    request::<settlement_models::PostSettlementModel, NoBody>(&mut self.sender, m).await?),
            }
        )
    }
}
