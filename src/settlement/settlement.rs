use serde::{Serialize, Deserialize};

#[cfg(feature = "typescript_types")]
use ts_rs::TS;

// TODO: is this actually u64? It's likely whatever type MySQL uses as an auto-incrementing
// integer.
#[cfg_attr(feature = "typescript_types", derive(TS))]
#[derive(Serialize, Deserialize, Debug)]
pub struct SettlementId(u64);
