use serde::{Serialize, Deserialize};
use fspiox_api::{Currency, FspId, DateTime};
use crate::central_ledger::participants::LedgerAccountType;
use crate::settlement::settlement::SettlementId;
use derive_more::{Display, FromStr};
use strum_macros::{ToString, EnumString};

#[cfg(feature = "typescript_types")]
use ts_rs::TS;

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, EnumString, ToString, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementWindowState {
    Open,
    Closed,
    PendingSettlement,
    Settled,
    Aborted,
    // TODO: doesn't exist in the spec but is the intermediate state returned immediately after
    // closure. Presumably this is the state that the window is put in before it is actually
    // closed. Raise an issue about this.
    Processing,
}

// TODO: is this actually u64? It's likely whatever type MySQL uses as an auto-incrementing
// integer.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, FromStr, Clone, Copy, Display)]
pub struct SettlementWindowId(u64);

// TODO: what.. is.. this? What is the settlement window content id? Is it actually the same as the
// settlementwindowid?
// Here's the spec: https://github.com/mojaloop/central-settlement/blob/e3c8cf8fc61543d1ab70880765ced23a9e98cb25/src/interface/swagger.json#L1135
// "integer"
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, FromStr, Clone, Copy, Display)]
pub struct SettlementWindowContentId(u64);

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettlementWindowContent {
    // TODO: is id the settlement window ID? Must be, right?
    pub id: SettlementWindowContentId,
    // TODO: not in the spec
    // https://github.com/mojaloop/central-settlement/blob/15d42ce259b3c1c57e81874c40ab5f5fb0981c6e/src/interface/swagger.json#L1134
    // Raise issue
    // Additionally, returned from POST /v2/settlements but not from GET /v2/settlements. Also, a
    // complete waste of space AFAICT.
    pub settlement_window_id: Option<SettlementWindowId>,
    pub state: SettlementWindowState,
    pub ledger_account_type: LedgerAccountType,
    pub currency_id: Currency,
    pub created_date: DateTime,
    pub changed_date: Option<DateTime>,
    // TODO: in spec, doesn't seem to be returned
    pub settlement_id: Option<SettlementId>,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettlementWindow {
    // Looks like the spec is incorrect here:
    // https://github.com/mojaloop/central-settlement/blob/15d42ce259b3c1c57e81874c40ab5f5fb0981c6e/src/interface/swagger.json#L1104
    // The actual response has this settlementWindowId property, rather than the `id` property
    // TODO: raise issue
    pub settlement_window_id: SettlementWindowId,
    pub reason: Option<String>,
    pub state: SettlementWindowState,
    pub created_date: DateTime,
    pub changed_date: Option<DateTime>,
    pub content: Option<Vec<SettlementWindowContent>>,
}

pub type SettlementWindows = Vec<SettlementWindow>;

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, ToString, EnumString, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SettlementWindowCloseState {
    Closed,
}

impl From<SettlementWindowCloseState> for SettlementWindowState {
    fn from(item: SettlementWindowCloseState) -> Self {
        match item {
            SettlementWindowCloseState::Closed => SettlementWindowState::Closed
        }
    }
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettlementWindowClosurePayload {
    // Yes, the spec says that this is required. And yes, the spec says there's only one value:
    // "CLOSED". It looks tricky to have it included by default with serde (i.e. without specifying
    // it in the struct itself, even though we actually have no use for it) and it seems like
    // overkill to make a custom implementation of serialize/deserialize. Additionally, serde
    // doesn't handle associated consts. Rust won't let us put it on this struct (which seems to
    // have been a conscious, reasoned decision)
    // TODO: when I tested whether serde handles associated consts, I may have done so incorrectly;
    // it's worth trying this again at some point.
    pub state: SettlementWindowCloseState,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CloseSettlementWindow {
    pub id: SettlementWindowId,
    pub payload: SettlementWindowClosurePayload,
}

#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlementWindows {
    pub currency: Option<Currency>,
    pub participant_id: Option<FspId>,
    pub state: Option<SettlementWindowState>,
    pub from_date_time: Option<DateTime>,
    pub to_date_time: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSettlementWindow {
    pub id: SettlementWindowId,
}

#[cfg(feature = "hyper")]
pub mod requests {
    use crate::settlement::settlement_windows::*;
    use fspiox_api::clients::NoBody;
    use crate::clients::requests::{get, post};

    // A PUT, you say? Yes I rather think so. But alas..
    impl From<CloseSettlementWindow> for http::Request<hyper::Body> {
        fn from(req: CloseSettlementWindow) -> http::Request<hyper::Body> {
            post(format!("/v2/settlementWindows/{}", req.id).as_str(), &req.payload)
        }
    }

    impl From<GetSettlementWindow> for http::Request<hyper::Body> {
        fn from(req: GetSettlementWindow) -> http::Request<hyper::Body> {
            get(format!("/v2/settlementWindows/{}", req.id).as_str(), &NoBody)
        }
    }

    impl From<GetSettlementWindows> for http::Request<hyper::Body> {
        fn from(req: GetSettlementWindows) -> http::Request<hyper::Body> {
            use itertools::Itertools;
            use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

            // https://url.spec.whatwg.org/#query-percent-encode-set
            const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'#');

            use std::collections::HashMap;
            let mut query_params: HashMap<&str, String> = HashMap::new();
            if let Some(c) = req.currency { query_params.insert("currency", c.to_string()); }
            if let Some(id) = &req.participant_id { query_params.insert("participantId", id.to_string()); }
            if let Some(st) = &req.state { query_params.insert("state", st.to_string()); }
            if let Some(from) = req.from_date_time { query_params.insert("fromDateTime", from.to_string()); }
            if let Some(to) = req.to_date_time { query_params.insert("toDateTime", to.to_string()); }
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
            get(format!("/v2/settlementWindows?{}", query_string).as_str(), &NoBody)
        }
    }
}
