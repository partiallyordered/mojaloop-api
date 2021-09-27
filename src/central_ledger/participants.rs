use serde::{Serialize, Deserialize};
use fspiox_api::{Currency, FspId, CorrelationId, Money, DateTime, Amount};
use derive_more::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

#[cfg(feature = "typescript_types")]
use ts_rs::TS;

// TODO:
// - consistency for derived traits
// - correct String vs &'static str etc. usage
// - module structure:
//   - in subdirectories like ./{name}/initialPositionAndLimits ?
//   - a macro for creating impl CentralLedgerRequest so we can sorta do this:
//       #[central_ledger_request(method = "POST", path = "/participants/{name}/initialPositionAndLimits")]
//     But how do we manage those path parameters? The path parameters are typed.
//     How do other request/server libraries handle this? Can we copy one of those? Or just _use one_? There's some argument that each of the requests
//     should just have its path parameters as a typed struct field, then it should have a
//     to_request method that turns it into a CentralLedgerRequest, with its path filled in with
//     actual values (i.e. /participants/some_fsp/accounts, rather than
//     /participants/{name}/accounts), or something..?)

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AccountId(u64);
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct SettlementAccountId(u64);

// TODO: contribute a PR to ts-rs to make this the default implementation for a newtype
#[cfg(feature = "typescript_types")]
impl TS for SettlementAccountId {
    fn name() -> String {
        "SettlementAccountId".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    fn decl() -> String {
        "type SettlementAccountId = number".to_string()
    }
}

// TODO: need custom deserializer here: https://stackoverflow.com/a/65576570
// Even better might be to serialize/deserialize as a boolean.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsActive {
    #[serde(rename = "1")]
    Yes,
    #[serde(rename = "0")]
    No,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HubAccountType {
    HubMultilateralSettlement,
    HubReconciliation,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum LedgerAccountType {
    Position,
    Settlement,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnyAccountType {
    Position,
    Settlement,
    HubMultilateralSettlement,
    HubReconciliation,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct HubAccount {
    #[cfg_attr(feature = "typescript_types", ts(rename = "type"))]
    pub r#type: HubAccountType,
    pub currency: Currency,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewParticipant {
    // TODO: is it possible to fill the created_by field optionally? It's returned by GET
    // /participants but I'm not sure it's possible to supply here.
    // pub created_by: Option<String>,
    // Looks like not:
    //   Error: Mojaloop API error: {"errorInformation":{"errorCode":"3101","errorDescription":"Malformed syntax - \"createdBy\" is not allowed"}}
    // But why? Is it something we're doing? Create an issue?
    pub name: FspId,
    // TODO: In the API, currency is optional. But it seems that attempting to create a participant
    // without a currency fails with an error "no hub currency account exists for this currency".
    // Probably should create an issue upstream.
    pub currency: Currency,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LimitType {
    NetDebitCap,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Limit {
    #[cfg_attr(feature = "typescript_types", ts(rename = "type"))]
    pub r#type: LimitType,
    // TODO: this should be Money, but "positive money"
    // Or should it be "positive integer money"?
    pub value: u32,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantLimit {
    #[cfg_attr(feature = "typescript_types", ts(rename = "type"))]
    pub r#type: LimitType,
    // TODO: this should be Money, but "positive money"
    // Or should it be "positive integer money"?
    pub value: u32,
    pub alarm_percentage: u8, // TODO: "number" in the spec. Probably needs to be [0,100].
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct InitialPositionAndLimits {
    pub currency: Currency,
    pub limit: Limit,
    // TODO: should this be Amount? Check the spec.
    pub initial_position: Amount,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ParticipantFundsInOutAction {
    RecordFundsIn,
    RecordFundsOutPrepareReserve,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFundsInOut {
    pub transfer_id: CorrelationId,
    pub action: ParticipantFundsInOutAction,
    pub external_reference: String, // From the spec, external_reference is only type: string, with no further validation
    pub reason: String, // From the spec, reason is only type: string, with no further validation
    pub amount: Money,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
pub enum PartyIdType {
    MSISDN,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantAccount {
    pub id: SettlementAccountId,
    pub ledger_account_type: AnyAccountType,
    pub currency: Currency,
    pub is_active: u8,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub name: FspId,
    pub id: String,
    // This should be DateTime, but the response comes back as a nested string. I.e.
    // { created: "\"2021-01-01T01:23:34Z\"" }
    #[serde(with = "serde_with::json::nested")]
    pub created: DateTime,
    pub is_active: u8,
    pub accounts: Vec<ParticipantAccount>,
}

pub type Participants = Vec<Participant>;

#[derive(Debug, Clone)]
pub struct GetParticipants { }

#[derive(Debug, Clone)]
pub struct PostHubAccount {
    pub account: HubAccount,
    pub name: FspId, // typically Hub or hub. TODO: a bit of documentation about where this is configured, and how a user can find it.
}

#[derive(Debug, Clone)]
pub struct PostParticipant {
    pub participant: NewParticipant,
}

#[derive(Debug, Clone)]
pub struct PostInitialPositionAndLimits {
    pub initial_position_and_limits: InitialPositionAndLimits,
    pub name: FspId,
}

// TODO: these are the same for hub and partipants except for the account types. It could be a good
// idea to have two different types. We'd then have one type HubAccount with ledger_account_type:
// HubAccountType and a DfspAccount type with ledger_account_type: LedgerAccountType, or perhaps
// DfspAccountType. And as a result from a GET /participants/accounts request, we could have an
// untagged enum type enum AnyAccount { HubAccount(HubAccount), DfspAccount(DfspAccount), } to
// enable us to parse with a single type.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct DfspAccount {
    pub id: SettlementAccountId,
    pub ledger_account_type: AnyAccountType,
    pub currency: Currency,
    // TODO: this is an enum with value 0 or 1. Probably use IsActive (see earlier in this file).
    pub is_active: u8,
    pub value: Amount,
    pub reserved_value: Amount,
    pub changed_date: DateTime,
}

pub type DfspAccounts = Vec<DfspAccount>;

pub type CallbackUrls = Vec<CallbackUrl>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCallbackUrls {
    pub name: FspId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDfspAccounts {
    pub name: FspId,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyIsActive {
    pub is_active: bool,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct NewParticipantLimit {
    pub currency: Currency,
    pub limit: ParticipantLimit,
}

#[derive(Debug, Clone)]
pub struct PutParticipantLimit {
    pub name: FspId,
    pub limit: NewParticipantLimit,
}

#[derive(Debug, Clone)]
pub struct GetParticipantLimits {
    pub name: FspId,
}

#[derive(Debug, Clone)]
pub struct PutParticipantAccount {
    pub name: FspId,
    pub account_id: SettlementAccountId,
    pub set_active: bool,
}

#[derive(Debug, Clone)]
pub struct PostParticipantSettlementFunds {
    pub account_id: SettlementAccountId,
    pub name: FspId,
    pub funds: ParticipantFundsInOut,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, EnumIter, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FspiopCallbackType {
    // The prefixes on these enums is fairly redundant, but mirrors the enums used in the API
    FspiopCallbackUrlParticipantBatchPut,
    FspiopCallbackUrlParticipantBatchPutError,
    FspiopCallbackUrlParticipantPut,
    FspiopCallbackUrlParticipantPutError,
    FspiopCallbackUrlPartiesGet,
    FspiopCallbackUrlPartiesPut,
    FspiopCallbackUrlPartiesPutError,
    FspiopCallbackUrlQuotes,
    FspiopCallbackUrlTransferError,
    FspiopCallbackUrlTransferPost,
    FspiopCallbackUrlTransferPut,
}

const fn get_callback_path(callback_type: FspiopCallbackType) -> &'static str {
    use FspiopCallbackType::*;
    match callback_type {
        FspiopCallbackUrlParticipantBatchPut      => "/participants/{{requestId}}",
        FspiopCallbackUrlParticipantBatchPutError => "/participants/{{requestId}}/error",
        FspiopCallbackUrlParticipantPut           => "/participants/{{partyIdType}}/{{partyIdentifier}}",
        FspiopCallbackUrlParticipantPutError      => "/participants/{{partyIdType}}/{{partyIdentifier}}/error",
        FspiopCallbackUrlPartiesGet               => "/parties/{{partyIdType}}/{{partyIdentifier}}",
        FspiopCallbackUrlPartiesPut               => "/parties/{{partyIdType}}/{{partyIdentifier}}",
        FspiopCallbackUrlPartiesPutError          => "/parties/{{partyIdType}}/{{partyIdentifier}}/error",
        FspiopCallbackUrlQuotes                   => "", // TODO: is this correct?
        FspiopCallbackUrlTransferError            => "/transfers/{{transferId}}/error",
        FspiopCallbackUrlTransferPost             => "/transfers",
        FspiopCallbackUrlTransferPut              => "/transfers/{{transferId}}",
    }
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackUrl {
    #[cfg_attr(feature = "typescript_types", ts(rename = "type"))]
    pub r#type: FspiopCallbackType,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct PostCallbackUrl {
    pub name: FspId,
    pub callback_type: FspiopCallbackType,
    pub hostname: String,
}

// TODO: usage of unwrap() on the result of the request builder tells us that we should replace
// many of our types that can _only_ accept ASCII [[32-126], [128-255]].
// https://docs.rs/http/0.2.4/http/header/struct.HeaderValue.html#method.from_bytes
// In fact, here we're really assuming that none of these values contain non-printable ASCII
// characters.

#[cfg(feature = "hyper")]
pub mod requests {
    use crate::central_ledger::participants::*;
    use fspiox_api::clients::NoBody;
    use crate::clients::requests::{get, post, put};

    impl From<GetParticipantLimits> for http::Request<hyper::Body> {
        fn from(req: GetParticipantLimits) -> http::Request<hyper::Body> {
            get(format!("/participants/{}/limits", req.name).as_str(), &NoBody)
        }
    }

    impl From<PutParticipantLimit> for http::Request<hyper::Body> {
        fn from(req: PutParticipantLimit) -> http::Request<hyper::Body> {
            put(format!("/participants/{}/limits", req.name).as_str(), &req.limit)
        }
    }

    impl From<PutParticipantAccount> for http::Request<hyper::Body> {
        fn from(req: PutParticipantAccount) -> http::Request<hyper::Body> {
            put(
                format!("/participants/{}/accounts/{}", req.name, req.account_id).as_str(),
                &CurrencyIsActive { is_active: req.set_active },
            )
        }
    }

    impl From<PostHubAccount> for http::Request<hyper::Body> {
        fn from(req: PostHubAccount) -> http::Request<hyper::Body> {
            post(format!("/participants/{}/accounts", req.name).as_str(), &req.account)
        }
    }

    impl From<PostCallbackUrl> for http::Request<hyper::Body> {
        fn from(req: PostCallbackUrl) -> http::Request<hyper::Body> {
            post(format!("/participants/{}/endpoints", req.name).as_str(), &CallbackUrl {
                r#type: req.callback_type,
                value: format!("{}{}", req.hostname, get_callback_path(req.callback_type)),
            })
        }
    }

    impl From<PostParticipantSettlementFunds> for http::Request<hyper::Body> {
        fn from(req: PostParticipantSettlementFunds) -> http::Request<hyper::Body> {
            post(format!("/participants/{}/accounts/{}", req.name, req.account_id).as_str(), &req.funds)
        }
    }

    impl From<GetParticipants> for http::Request<hyper::Body> {
        fn from(_req: GetParticipants) -> http::Request<hyper::Body> {
            get("/participants", &NoBody)
        }
    }

    impl From<GetCallbackUrls> for http::Request<hyper::Body> {
        fn from(req: GetCallbackUrls) -> http::Request<hyper::Body> {
            get(format!("/participants/{}/endpoints", req.name).as_str(), &NoBody)
        }
    }

    impl From<GetDfspAccounts> for http::Request<hyper::Body> {
        fn from(req: GetDfspAccounts) -> http::Request<hyper::Body> {
            get(format!("/participants/{}/accounts", req.name).as_str(), &NoBody)
        }
    }

    impl From<PostInitialPositionAndLimits> for http::Request<hyper::Body> {
        fn from(req: PostInitialPositionAndLimits) -> http::Request<hyper::Body> {
            post(
                format!("/participants/{}/initialPositionAndLimits", req.name).as_str(),
                &req.initial_position_and_limits
            )
        }
    }

    impl From<PostParticipant> for http::Request<hyper::Body> {
        fn from(req: PostParticipant) -> http::Request<hyper::Body> {
            post("/participants", &req.participant)
        }
    }
}
