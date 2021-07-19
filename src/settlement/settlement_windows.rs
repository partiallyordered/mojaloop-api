use serde::{Serialize, Deserialize};
use crate::common::{Method, MojaloopService};
pub use crate::common::MojaloopRequest;
use fspiox_api::common::{Currency, FspId, DateTime};
use crate::central_ledger::participants::LedgerAccountType;
use crate::settlement::settlement::SettlementId;
use derive_more::{Display, FromStr};
use itertools::Itertools;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use strum_macros::EnumString;

// https://url.spec.whatwg.org/#query-percent-encode-set
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'#');

#[derive(Serialize, Deserialize, Debug, Display, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementWindowState {
    Open,
    Closed,
    PendingSettlement,
    Settled,
    Aborted,
}

// TODO: is this actually u64? It's likely whatever type MySQL uses as an auto-incrementing
// integer.
#[derive(Serialize, Deserialize, Debug, FromStr)]
pub struct SettlementWindowId(u64);

// TODO: what.. is.. this? What is the settlement window content id? Is it actually the same as the
// settlementwindowid?
// Here's the spec: https://github.com/mojaloop/central-settlement/blob/e3c8cf8fc61543d1ab70880765ced23a9e98cb25/src/interface/swagger.json#L1135
// "integer"
#[derive(Serialize, Deserialize, Debug)]
pub struct SettlementWindowContentId(u64);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementWindowContent {
    // TODO: is id the settlement window ID? Must be, right?
    pub id: SettlementWindowContentId,
    pub state: SettlementWindowState,
    pub ledger_account_type: LedgerAccountType,
    pub currency_id: Currency,
    pub created_date: DateTime,
    pub changed_date: Option<DateTime>,
    pub settlement_id: Option<SettlementId>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementWindow {
    pub id: SettlementWindowId,
    pub reason: Option<String>,
    pub state: SettlementWindowState,
    pub created_date: DateTime,
    pub changed_date: Option<DateTime>,
    pub content: Option<Vec<SettlementWindowContent>>,
}

pub type SettlementWindows = Vec<SettlementWindow>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlementWindows {
    pub currency: Option<Currency>,
    pub participant_id: Option<FspId>,
    pub state: Option<SettlementWindowState>,
    pub from_date_time: Option<DateTime>,
    pub to_date_time: Option<DateTime>,
}

impl MojaloopRequest<(), SettlementWindows> for GetSettlementWindows {
    const METHOD: Method = Method::GET;
    const SERVICE: MojaloopService = MojaloopService::CentralSettlement;

    fn path(&self) -> String {
        use std::collections::HashMap;
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(c) = self.currency { query_params.insert("currency", c.to_string()); }
        if let Some(id) = &self.participant_id { query_params.insert("participantId", id.to_string()); }
        if let Some(st) = &self.state { query_params.insert("state", st.to_string()); }
        if let Some(from) = self.from_date_time { query_params.insert("fromDateTime", from.to_string()); }
        if let Some(to) = self.to_date_time { query_params.insert("toDateTime", to.to_string()); }
        // TODO: this assert isn't great, we'd prefer correct by construction, if possible
        assert!(query_params.len() > 0, "At least one GET /settlementWindows query parameter is required");
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
        format!("/settlementWindows?{}", query_string)
    }

    fn body(&self) -> Option<()> { None }
}
