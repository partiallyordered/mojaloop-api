use serde::{Serialize, Deserialize};

// TODO: is this actually u64? It's likely whatever type MySQL uses as an auto-incrementing
// integer.
#[derive(Serialize, Deserialize, Debug)]
pub struct SettlementId(u64);
