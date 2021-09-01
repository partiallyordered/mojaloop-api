use serde::{Serialize, Deserialize};
use crate::common::{Method, MojaloopService};
use fspiox_api::common::{Amount, Currency, FspId, DateTime};
use crate::settlement::settlement_windows::{SettlementWindowId, SettlementWindowState, SettlementWindowContent};
pub use crate::common::MojaloopRequest;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use strum_macros::{EnumString, ToString};
use itertools::Itertools;

#[cfg(feature = "typescript_types")]
use ts_rs::TS;

// https://url.spec.whatwg.org/#query-percent-encode-set
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'#');

// TODO: is this actually u64? It's likely whatever type MySQL uses as an auto-incrementing
// integer.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
pub struct SettlementId(u64);

// TODO: is this actually u64? It's likely whatever type MySQL uses as an auto-incrementing
// integer.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
pub struct ParticipantId(u64);

// TODO: is this actually u64? It's likely whatever type MySQL uses as an auto-incrementing
// integer.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
pub struct ParticipantCurrencyId(u64);

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, EnumString, ToString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementState {
    PendingSettlement,
    PsTransfersRecorded,
    PsTransfersReserved,
    PsTransfersCommitted,
    Settling,
    Settled,
    Aborted,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementSettlementWindow {
    // Note that we have this struct: `SettlementSettlementWindow` and the `SettlementWindow`
    // struct which are almost the same. The difference is the `id` vs. `settlement_window_id`
    // properties. The spec states the property should be called `id`, and when returning from GET
    // /settlements, it is. But when returning from GET /settlementWindows the property is named
    // `settlementWindowId`. Therefore, we have two different structs for parsing the results here.
    // TODO: raise issue
    pub id: SettlementWindowId,
    pub reason: Option<String>,
    pub state: SettlementWindowState,
    pub created_date: DateTime,
    pub changed_date: Option<DateTime>,
    pub content: Option<Vec<SettlementWindowContent>>,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetSettlementAmount {
    pub amount: Amount,
    pub currency: Currency,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementAccount {
    pub id: ParticipantCurrencyId,
    pub reason: String,
    // TODO: spec should be updated here: https://github.com/mojaloop/central-settlement/blob/15d42ce259b3c1c57e81874c40ab5f5fb0981c6e/src/interface/swagger.json#L1010
    // Raise issue/PR
    pub state: SettlementState,
    pub net_settlement_amount: NetSettlementAmount,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementParticipant {
    pub id: ParticipantId,
    pub accounts: Vec<SettlementAccount>,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settlement {
    pub id: SettlementId,
    pub state: SettlementState,
    // TODO: not in spec
    // https://github.com/mojaloop/central-settlement/blob/15d42ce259b3c1c57e81874c40ab5f5fb0981c6e/src/interface/swagger.json#L1202
    // Raise issue
    pub created_date: DateTime,
    pub changed_date: DateTime,
    pub settlement_windows: Vec<SettlementSettlementWindow>,
    pub participants: Vec<SettlementParticipant>,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlements {
    pub currency: Option<Currency>,
    pub participant_id: Option<FspId>,
    pub settlement_window_id: Option<SettlementWindowId>,
    pub state: Option<SettlementState>,
    pub from_date_time: Option<DateTime>,
    pub to_date_time: Option<DateTime>,
    pub from_settlement_window_date_time: Option<DateTime>,
    pub to_settlement_window_date_time: Option<DateTime>,
    // TODO: accountId
    // Didn't do it at the time of writing because I don't know if there's a type for it
    // Spec: https://github.com/mojaloop/central-settlement/blob/15d42ce259b3c1c57e81874c40ab5f5fb0981c6e/src/interface/swagger.json#L267
}

pub type Settlements = Vec<Settlement>;

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSettlement {
    /// The name of the settlement model
    pub settlement_model: String,
    pub reason: String,
    pub settlement_windows: Vec<SettlementWindowId>,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostSettlement {
    pub new_settlement: NewSettlement,
}

impl MojaloopRequest<NewSettlement, Settlement> for PostSettlement {
    const METHOD: Method = Method::POST;
    const SERVICE: MojaloopService = MojaloopService::CentralSettlement;
    fn path(&self) -> String { format!("/v2/settlement") }
    fn body(&self) -> Option<NewSettlement> { Some(self.new_settlement.clone()) }
}

impl MojaloopRequest<(), Settlements> for GetSettlements {
    const METHOD: Method = Method::GET;
    const SERVICE: MojaloopService = MojaloopService::CentralSettlement;

    fn path(&self) -> String {
        use std::collections::HashMap;
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(c) = self.currency { query_params.insert("currency", c.to_string()); }
        if let Some(pid) = &self.participant_id { query_params.insert("participantId", pid.to_string()); }
        if let Some(swid) = &self.settlement_window_id { query_params.insert("settlementWindowId", swid.to_string()); }
        if let Some(st) = &self.state { query_params.insert("state", st.to_string()); }
        if let Some(from) = self.from_date_time { query_params.insert("fromDateTime", from.to_string()); }
        if let Some(to) = self.to_date_time { query_params.insert("toDateTime", to.to_string()); }
        if let Some(sw_from) = self.from_settlement_window_date_time { query_params.insert("fromSettlementWindowDateTime", sw_from.to_string()); }
        if let Some(sw_to) = self.to_settlement_window_date_time { query_params.insert("toSettlementWindowDateTime", sw_to.to_string()); }
        // TODO: this assert isn't great, we'd prefer correct by construction, if possible
        assert!(query_params.len() > 0, "At least one GET /settlements query parameter is required");
        let query_string = format!(
            "{}",
            query_params
                .iter()
                .map(|(k, v)|
                    format!(
                        "{}={}",
                        utf8_percent_encode(k, &QUERY_ENCODE_SET),
                        utf8_percent_encode(v, &QUERY_ENCODE_SET),
                    )
                )
                .format("&")
        );
        format!("/v2/settlements?{}", query_string)
    }

    fn body(&self) -> Option<()> { None }
}
