use serde::{Serialize, Deserialize};
use fspiox_api::common::{Currency,FspId,CorrelationId,Money};
use crate::common::Method;
pub use crate::common::CentralLedgerRequest;

// TODO:
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountId(u64);
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementAccountId(u64);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewParticipant {
    pub name: FspId,
    pub currency: Currency,
}

pub struct PostParticipant {
    pub participant: NewParticipant,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LimitType {
    NetDebitCap,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Limit {
    // This is a "raw" identifier, which allows us to use most lang keywords here
    pub r#type: LimitType,
    // TODO: this should be Money, but "positive money"
    // Or should it be "positive integer money"?
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct InitialPositionAndLimits {
    pub currency: Currency,
    pub limit: Limit,
    pub initial_position: Money,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum FundsInOutAction {
    RecordFundsIn,
    RecordFundsOutPrepareReserve,
    RecordFundsOutCommit,
    RecordFundsOutAbort,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundsInOut {
    pub transfer_id: CorrelationId,
    pub action: FundsInOutAction,
    pub external_reference: String, // From the spec, external_reference is only type: string, with no further validation
    pub reason: String, // From the spec, reason is only type: string, with no further validation
    pub amount: Money,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PartyIdType {
    MSISDN,
}

pub struct PostInitialPositionAndLimits {
    pub initial_position_and_limits: InitialPositionAndLimits,
    pub name: FspId,
}

pub struct GetDfspAccounts {
    pub name: FspId,
}

pub struct PostParticipantSettlementFunds {
    pub account_id: SettlementAccountId,
    pub name: FspId,
    pub funds: FundsInOut,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FspiopCallbackType {
    ParticipantBatchPut,
    ParticipantBatchPutError,
    ParticipantPut,
    ParticipantPutError,
    PartiesGet,
    PartiesPut,
    PartiesPutError,
    Quotes,
    TransferError,
    TransferPost,
    TransferPut,
}

const fn get_callback_path(callback_type: FspiopCallbackType) -> &'static str {
    use FspiopCallbackType::*;
    match callback_type {
        ParticipantBatchPut      => "/participants/{{requestId}}",
        ParticipantBatchPutError => "/participants/{{requestId}}/error",
        ParticipantPut           => "/participants/{{partyIdType}}/{{partyIdentifier}}",
        ParticipantPutError      => "/participants/{{partyIdType}}/{{partyIdentifier}}/error",
        PartiesGet               => "/parties/{{partyIdType}}/{{partyIdentifier}}",
        PartiesPut               => "/parties/{{partyIdType}}/{{partyIdentifier}}",
        PartiesPutError          => "/parties/{{partyIdType}}/{{partyIdentifier}}/error",
        Quotes                   => "", // TODO: is this correct?
        TransferError            => "/transfers/{{transferId}}/error",
        TransferPost             => "/transfers",
        TransferPut              => "/transfers/{{transferId}}",
    }
}

pub trait PostCallbackUrl {
    const CALLBACK_TYPE: FspiopCallbackType;
    fn get_name(&self) -> &String;
    // TODO: should be something like:
    // fn get_hostname(&self) -> &URI;
    fn get_hostname(&self) -> &String;
}

pub struct PostCallbackUrlParticipantBatchPutError {
    pub party_id_type: PartyIdType,
    pub name: String,
    pub hostname: String
}

impl PostCallbackUrl for PostCallbackUrlParticipantBatchPutError {
    const CALLBACK_TYPE: FspiopCallbackType = FspiopCallbackType::ParticipantBatchPutError;
    fn get_name(&self) -> &String { &self.name }
    fn get_hostname(&self) -> &String { &self.hostname }
}

pub struct PostCallbackUrlParticipantBatchPut {
    pub party_id_type: PartyIdType,
    pub name: String,
    pub hostname: String,
}

impl PostCallbackUrl for PostCallbackUrlParticipantBatchPut {
    const CALLBACK_TYPE: FspiopCallbackType = FspiopCallbackType::ParticipantBatchPut;
    fn get_name(&self) -> &String { &self.name }
    fn get_hostname(&self) -> &String { &self.hostname }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackUrl {
    r#type: FspiopCallbackType,
    value: String,
}

pub struct PostCbUrl {
    pub name: String,
    pub callback_type: FspiopCallbackType,
    pub hostname: String,
}

impl CentralLedgerRequest<CallbackUrl> for PostCbUrl {
    // Wondering if this should be a PUT? Yes. Yes it should. From the spec:
    // > Add/Update participant endpoints
    // https://github.com/mojaloop/central-ledger/blob/52b7494c9ec1160d9ab4427b05e6a12283a848f7/src/api/interface/swagger.json#L399
    const METHOD: Method = Method::POST;
    fn path(&self) -> String {
        format!("/participants/{:?}/endpoints", self.name)
    }
    fn body(&self) -> CallbackUrl {
        CallbackUrl {
            r#type: self.callback_type,
            value: format!("{:?}{:?}", self.hostname, get_callback_path(self.callback_type)),
        }
    }
}

impl CentralLedgerRequest<FundsInOut> for PostParticipantSettlementFunds {
    const METHOD: Method = Method::POST;
    fn path(&self) -> String {
        format!("/participants/{:?}/accounts/{:?}", self.name, self.account_id)
    }
    fn body(&self) -> FundsInOut { self.funds.clone() }
}

impl CentralLedgerRequest<Option<String>> for GetDfspAccounts {
    const METHOD: Method = Method::GET;
    fn path(&self) -> String {
        format!("/participants/{:?}/accounts", self.name)
    }
    fn body(&self) -> Option<String> { None }
}

impl CentralLedgerRequest<InitialPositionAndLimits> for PostInitialPositionAndLimits {
    const METHOD: Method = Method::POST;
    fn path(&self) -> String {
        format!("/participants/{:?}/initialPositionAndLimits", self.name)
    }
    fn body(&self) -> InitialPositionAndLimits { self.initial_position_and_limits }
}

impl CentralLedgerRequest<NewParticipant> for PostParticipant {
    const METHOD: Method = Method::POST;
    fn path(&self) -> String {
        "/participants".to_string()
    }
    fn body(&self) -> NewParticipant { self.participant.clone() }
}
