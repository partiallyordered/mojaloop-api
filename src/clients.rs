pub mod central_ledger;
pub mod settlement;
pub use fspiox_api::clients::*;

pub(crate) mod requests {
    pub fn base<T: serde::Serialize>(
        path: &str, body: &T, method: http::Method
    ) -> hyper::Request<hyper::body::Body> {
        hyper::Request::builder()
            .uri(path)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .method(method)
            .body(hyper::Body::from(serde_json::to_string(body).unwrap()))
            .unwrap()
    }

    pub fn get<T: serde::Serialize>(path: &str, body: &T) -> hyper::Request<hyper::body::Body> {
        base(path, body, http::Method::GET)
    }

    pub fn post<T: serde::Serialize>(path: &str, body: &T) -> hyper::Request<hyper::body::Body> {
        base(path, body, http::Method::POST)
    }

    pub fn put<T: serde::Serialize>(path: &str, body: &T) -> hyper::Request<hyper::body::Body> {
        base(path, body, http::Method::PUT)
    }
}

pub mod k8s {
    pub use fspiox_api::clients::k8s::*;

    use super::Result;
    use fspiox_api::clients::{transfer, quote, FspiopClient};
    use crate::clients::{central_ledger, settlement};

    // Shadow the fspiox-api implementation
    pub struct Clients {
        pub transfer: transfer::Client,
        pub quote: quote::Client,
        pub central_ledger: central_ledger::Client,
        pub settlement: settlement::Client,
    }

    // Shadow the fspiox-api implementation
    pub async fn get_all_from_k8s(
        client: Option<kube::client::Client>,
        namespace: &Option<String>,
    ) -> Result<Clients> {
        let client = ensure_client(client).await?;
        let (transfer, quote, central_ledger, settlement) = tokio::try_join!(
            transfer::Client::from_k8s_params(Some(client.clone()), namespace),
            quote::Client::from_k8s_params(Some(client.clone()), namespace),
            central_ledger::Client::from_k8s_params(Some(client.clone()), namespace),
            settlement::Client::from_k8s_params(Some(client), namespace),
        )?;
        Ok(Clients { transfer, quote, central_ledger, settlement })
    }
}
