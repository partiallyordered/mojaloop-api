use serde::{Serialize, Deserialize};
use fspiox_api::common::{Currency,FspId};
use crate::common::Method;
pub use crate::common::CentralLedgerRequest;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct NewParticipant {
    pub name: FspId,
    pub currency: Currency,
}

pub struct PostParticipant {
    pub participant: NewParticipant,
}

impl CentralLedgerRequest for PostParticipant {
    const PATH: &'static str = "/participants";
    const METHOD: Method = Method::POST;
}
