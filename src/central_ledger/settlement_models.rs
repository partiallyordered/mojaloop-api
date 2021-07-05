use serde::{Serialize, Deserialize};
use crate::common::Method;
pub use crate::common::CentralLedgerRequest;
use derive_more::Display;

// https://github.com/mojaloop/central-ledger/blob/01435fda1d61093b2e20ff2385e8d65393dac640/src/api/interface/swagger.json#L1214
// > Create a settlement model. This will create any associated ledgerAccountTypes for every
// > participant that matches the settlementModel's currency

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementGranularity {
    Gross,
    Net,
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementInterchange {
    Bilateral,
    Multilateral,
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementDelay {
    Deferred,
    Immediate,
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LedgerAccountType {
    InterchangeFee,
    Position,
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementAccountType {
    Position,
    Settlement,
    InterchangeFeeSettlement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettlementModel {
    pub auto_position_reset: bool,
    pub ledger_account_type: LedgerAccountType,
    pub settlement_account_type: SettlementAccountType,
    // TODO: validation, 2-30 alphanum characters
    // https://github.com/mojaloop/central-ledger/blob/01435fda1d61093b2e20ff2385e8d65393dac640/src/api/interface/swagger.json#L1583
    pub name: String,
    pub require_liquidity_check: bool,
    pub settlement_delay: SettlementDelay,
    pub settlement_granularity: SettlementGranularity,
    pub settlement_interchange: SettlementInterchange,
    pub currency: fspiox_api::common::Currency,
}

#[derive(Debug)]
pub struct PostSettlementModel {
    pub settlement_model: SettlementModel,
}

impl CentralLedgerRequest<SettlementModel, ()> for PostSettlementModel {
    const METHOD: Method = Method::POST;
    fn path(&self) -> String { format!("/settlementModels") }
    fn body(&self) -> Option<SettlementModel> { Some(self.settlement_model.clone()) }
}
