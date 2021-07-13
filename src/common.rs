use fspiox_api::common::ErrorResponse;
use strum_macros::EnumString;
use thiserror::Error;
#[cfg(feature = "reqwest")]
use reqwest;

// TODO: replace with http::method::Method enums. These are in common use amongst various HTTP
// crates.
#[derive(EnumString, strum_macros::Display, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
}

#[derive(Debug)]
pub enum MojaloopService {
    CentralLedger,
    CentralSettlement,
}

impl From<Method> for http::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::GET => http::Method::GET,
            Method::POST => http::Method::POST,
            Method::PUT => http::Method::PUT,
        }
    }
}

pub enum RequestError {
    Mojaloop(fspiox_api::common::ErrorResponse),
    InvalidJson,
    #[cfg(feature = "reqwest")]
    Connection(reqwest::Error),
}

// TODO: the trait bound on serde::Serialize should probably be behind a feature
// TODO: should MojaloopRequest implement Serialize and Deserialize?
pub trait MojaloopRequest<RequestBody, ResponseBody>
where
    RequestBody: serde::Serialize,
    ResponseBody: serde::de::DeserializeOwned,
{
    // TODO: a constant function to parse a path and validate it. This means any errors specifying
    // a valid path can be caught at compile time instead of runtime. Look at the implementation of
    // http::uri::PathAndQuery. See: https://doc.rust-lang.org/reference/const_eval.html
    // This might also be interesting: https://api.rocket.rs/v0.5-rc/rocket/macro.uri.html
    // Could also just test each implementation to check its path parses correctly as a URI.
    // TODO: probably should be &String, or strictly a URI.
    // https://docs.rs/http/0.1.3/http/uri/index.html
    fn path(&self) -> String;

    // TODO: GET requests don't have a body, should/can we make the body optional? Do we need
    // different types, i.e. MojaloopRequest or CentralLedgerPostRequest? Should we
    // have a body_json and/or body_string method?
    fn body(&self) -> Option<RequestBody>;

    fn body_json(&self) -> String {
        match self.body() {
            None => "".to_string(),
            Some(body) => serde_json::to_string(&body).unwrap()
        }
    }

    // TODO: replace with http::method::Method enums. These are in common use amongst various HTTP
    // crates.
    const METHOD: Method;
    const CONTENT_TYPE: &'static str = "application/json";
    const ACCEPT: &'static str = "application/json";
    const SERVICE: MojaloopService;

    // TODO: it's probably possible (and likely desirable) to move this function outside the trait.
    // This would allow us to have an associated type for each MojaloopRequest. We could then
    // refer to the response type like request::ResponseType for things like deserializing.
    //
    // We still can't impose the type constraint that the associated type must implement
    // serde::Deserialize until GATs land, but that's arguably not strictly required for
    // implementation of the MojaloopRequest trait (it _is_ a JSON API so we _should_ be able
    // to de/serialize all requests and responses from/to json, but that _can_ be enforced in the
    // type constraints for this function, instead of on the trait).
    //
    // After all, the main goal of this trait was to correlate requests and responses.
    //
    // The only reason this wasn't done at the time of writing was the need to propagate this
    // change through dependencies.
    fn deserialize_response(body: &str) -> Result<ResponseBody, RequestError> {
        serde_json::from_str::<ResponseBody>(body)
            .or_else(|_| {
                match serde_json::from_str::<ErrorResponse>(body) {
                    Ok(ml_api_err) => Err(RequestError::Mojaloop(ml_api_err)),
                    // TODO: this could potentially be that we have an invalid struct that serde
                    // cannot deserialize into, not just invalid JSON. We should consider handling
                    // this scenario also.
                    Err(_) => Err(RequestError::InvalidJson),
                }
            })
    }
}

#[derive(Debug)]
pub enum Response<T: serde::de::DeserializeOwned> {
    ResponseBody(T),
}

#[derive(Debug, Error)]
pub enum MlApiErr {
    #[error("Couldn't parse string response. Message: {0}")]
    CouldNotParseResponse(String),
    #[error("Mojaloop API error: {0}")]
    MojaloopApiError(fspiox_api::common::ErrorResponse),
}

fn fail<RespBody, E: std::fmt::Display>(bs: &[u8], e: E) -> Result<RespBody, MlApiErr> {
    let str_response = std::str::from_utf8(bs).unwrap();
    println!("Failed to parse response {}. Error message: {}.", str_response, e);
    Err(MlApiErr::CouldNotParseResponse(str_response.to_string()))
}

pub fn resp_headers_from_raw_http(bs: &[u8]) ->
    Result<(http::Response<()>, usize), MlApiErr>
{
    let mut raw_headers = [httparse::EMPTY_HEADER; 64];
    let mut http_resp = httparse::Response::new(&mut raw_headers);

    let body_index =
        match http_resp.parse(bs).or_else(|e| fail(bs, e))? {
            httparse::Status::Partial => fail(bs, "Partial HTTP header received"),
            httparse::Status::Complete(first_body_byte) => Ok(first_body_byte),
        }?;

    let mut result = http::Response::builder().status(http_resp.code.unwrap());
    let headers = result.headers_mut().unwrap();

    for raw_header in http_resp.headers.iter() {
        headers.insert(
            http::header::HeaderName::from_bytes(raw_header.name.as_bytes()).unwrap(),
            http::HeaderValue::from_bytes(raw_header.value).unwrap(),
        );
    }

    Ok((
        result.body(()).unwrap(),
        body_index,
    ))
}

// TODO: this should perhaps be feature gated behind the httparse crate. Though that crate is
// rather small and doesn't have transitive dependencies.
pub fn resp_from_raw_http<RespBody>(bs: &[u8]) -> Result<RespBody, MlApiErr>
where
    RespBody: serde::de::DeserializeOwned,
{
    let (http_resp, body_index) = resp_headers_from_raw_http(bs)?;

    match http_resp.status().as_u16() {
        200..=299 => serde_json::from_slice::<RespBody>(&bs[body_index..]).or_else(|e| fail(bs, e)),
        400..=599 =>
            match serde_json::from_slice::<fspiox_api::common::ErrorResponse>(&bs[body_index..]) {
                Ok(ml_api_err) => Err(MlApiErr::MojaloopApiError(ml_api_err)),
                Err(m) => fail(bs, m),
            },
        c => panic!("Handled 200-299, 400-599 responses. Received {}.", c)
    }
}

pub fn req_to_raw_http<RqBody, RespBody, CLR>(req: CLR) -> Vec<u8>
where
    RqBody: serde::Serialize,
    RespBody: serde::de::DeserializeOwned,
    CLR: MojaloopRequest<RqBody,RespBody>,
{
    // TODO: probably a good idea to calculate the content length and supply it. It is, however,
    // optional, so not urgent.
    let body = match CLR::METHOD {
        Method::GET => "".to_string(),
        Method::PUT | Method::POST => serde_json::to_string(&req.body()).unwrap(),
    };
    format!(
        "{} {} HTTP/1.1\r\ncontent-type: {}\r\naccept: {}\r\ncontent-length:{}\r\n\r\n{}",
        CLR::METHOD,
        req.path(),
        CLR::CONTENT_TYPE,
        CLR::ACCEPT,
        body.len(),
        body,
    ).as_bytes().to_owned()
}

#[cfg(feature = "hyper")]
pub fn to_hyper_request<RqBody, RespBody, CLR>(req: CLR) -> Result<hyper::Request<hyper::body::Body>, http::Error>
where
    RqBody: serde::Serialize,
    RespBody: serde::de::DeserializeOwned,
    CLR: MojaloopRequest<RqBody,RespBody>,
{
    let body = match CLR::METHOD {
        Method::GET => "".to_string(),
        Method::PUT | Method::POST => serde_json::to_string(&req.body()).unwrap(),
    };
    // TODO: replace with http::method::Method enums. These are in common use amongst various HTTP
    // crates.
    let method = match CLR::METHOD {
        Method::GET => http::method::Method::GET,
        Method::PUT => http::method::Method::PUT,
        Method::POST => http::method::Method::POST,
    };
    hyper::Request::builder()
        .uri(req.path())
        .method(method)
        .body(hyper::Body::from(body))
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
    R: MojaloopRequest<RqBody,RespBody>,
{
    let rq = match R::METHOD {
        Method::POST => {
            c.post(format!("{}{}", host, r.path())).json(&r.body())
        },
        Method::GET => {
            c.get(format!("{}{}", host, r.path()))
        },
        Method::PUT => {
            c.put(format!("{}{}", host, r.path())).json(&r.body())
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
