use fspiox_api::common::ErrorResponse;
#[cfg(feature = "reqwest")]
use reqwest;

pub enum Method {
    GET,
    POST,
}

pub enum RequestError {
    Mojaloop(fspiox_api::common::ErrorResponse),
    InvalidJson,
    #[cfg(feature = "reqwest")]
    Connection(reqwest::Error),
}

// TODO: the trait bound on serde::Serialize should probably be behind a feature
// TODO: should CentralLedgerRequest implement Serialize and Deserialize?
pub trait CentralLedgerRequest<RequestBody: serde::Serialize, ResponseBody: serde::de::DeserializeOwned> {
    // TODO: a constant function to parse a path and validate it. This means any errors specifying
    // a valid path can be caught at compile time instead of runtime. Look at the implementation of
    // http::uri::PathAndQuery. See: https://doc.rust-lang.org/reference/const_eval.html

    fn path(&self) -> String;

    // TODO: GET requests don't have a body, should/can we make the body optional? Do we need
    // different types, i.e. CentralLedgerRequestWithBody or CentralLedgerPostRequest?
    fn body(&self) -> RequestBody;

    const METHOD: Method;
    const CONTENT_TYPE: &'static str = "application/json";
    const ACCEPT: &'static str = "application/json";

    fn deserialize_response(body: &str) -> Result<ResponseBody, RequestError> {
        serde_json::from_str::<ResponseBody>(body)
            .or_else(|_| {
                match serde_json::from_str::<ErrorResponse>(body) {
                    Ok(ml_api_err) => Err(RequestError::Mojaloop(ml_api_err)),
                    Err(_) => Err(RequestError::InvalidJson),
                }
            })
    }
}

#[cfg(feature = "reqwest")]
pub async fn send<RqBody, RespBody, R>(
    c: &reqwest::Client,
    host: &str,
    r: R,
) -> Result<RespBody, RequestError>
where
    RqBody: serde::Serialize,
    RespBody: serde::de::DeserializeOwned,
    R: CentralLedgerRequest<RqBody,RespBody>,
{
    let rq = match R::METHOD {
        Method::POST => {
            c.post(format!("{}{}", host, r.path())).json(&r.body())
        },
        Method::GET => {
            c.get(format!("{}{}", host, r.path()))
        },
    };

    let body_text = rq
        .send().await
        .map_err(|err: reqwest::Error| RequestError::Connection(err))?
        .text().await
        .map_err(|_| RequestError::InvalidJson)?;

    // TODO: we should check the response status to decide what to deserialize, instead of trying
    // to deserialize the response, then an error

    R::deserialize_response(&body_text)

}
