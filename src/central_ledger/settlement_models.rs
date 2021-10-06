use serde::{Serialize, Deserialize};
use derive_more::Display;
use strum_macros::EnumString;

#[cfg(feature = "typescript_types")]
use ts_rs::TS;

// https://github.com/mojaloop/central-ledger/blob/01435fda1d61093b2e20ff2385e8d65393dac640/src/api/interface/swagger.json#L1214
// > Create a settlement model. This will create any associated ledgerAccountTypes for every
// > participant that matches the settlementModel's currency

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum SettlementGranularity {
    Gross,
    Net,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum SettlementInterchange {
    Bilateral,
    Multilateral,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum SettlementDelay {
    Deferred,
    Immediate,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum LedgerAccountType {
    InterchangeFee,
    Position,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
pub enum SettlementAccountType {
    // Spec says POSITION, but this is not valid
    // TODO: PR/Slack message/GH issue
    // https://github.com/mojaloop/central-ledger/blob/01435fda1d61093b2e20ff2385e8d65393dac640/src/api/interface/swagger.json#L1619
    // Position,
    Settlement,
    InterchangeFeeSettlement,
}

// TODO: validation, 2-30 alphanum characters
// https://github.com/mojaloop/central-ledger/blob/01435fda1d61093b2e20ff2385e8d65393dac640/src/api/interface/swagger.json#L1583
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, Display)]
pub struct SettlementModelName(arrayvec::ArrayString<30>);

impl SettlementModelName {
    // TODO: can this be compile-time?
    pub fn from(item: &str) -> Result<Self, arrayvec::CapacityError<&str>> {
        Ok(SettlementModelName(arrayvec::ArrayString::from(item)?))
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Hash, PartialEq, Eq, Display)]
pub enum ParseSettlementModelNameErr {
    SettlementModelNameTooLong,
}

impl core::str::FromStr for SettlementModelName {
    type Err = ParseSettlementModelNameErr;

    // TODO: can this be compile-time for 'static &str?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SettlementModelName::from(s).map_err(|_| ParseSettlementModelNameErr::SettlementModelNameTooLong)
    }
}

#[cfg(feature = "typescript_types")]
impl TS for SettlementModelName {
    fn name() -> String {
        "SettlementModelName".to_string()
    }

    fn dependencies() -> Vec<(std::any::TypeId, String)> {
        Vec::new()
    }

    fn transparent() -> bool { false }

    // TODO: needs to have a size limit somehow. We could perhaps write out a whole class or
    // interface definition here. Could that work somehow?
    fn decl() -> String {
        "type SettlementModelName = string".to_string()
    }
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct SettlementModel {
    pub auto_position_reset: bool,
    pub ledger_account_type: LedgerAccountType,
    pub settlement_account_type: SettlementAccountType,
    pub name: SettlementModelName,
    pub require_liquidity_check: bool,
    pub settlement_delay: SettlementDelay,
    pub settlement_granularity: SettlementGranularity,
    pub settlement_interchange: SettlementInterchange,
    pub currency: fspiox_api::Currency,
}

#[derive(Debug, Clone, Copy)]
pub struct PostSettlementModel {
    pub settlement_model: SettlementModel,
}

#[cfg(feature = "hyper")]
pub mod requests {
    use crate::central_ledger::settlement_models::*;
    use crate::clients::requests::post;

    impl From<PostSettlementModel> for http::Request<hyper::Body> {
        fn from(req: PostSettlementModel) -> http::Request<hyper::Body> {
            post("/settlementModels", &req.settlement_model)
        }
    }
}
