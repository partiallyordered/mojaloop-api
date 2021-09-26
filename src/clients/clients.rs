use serde::Deserialize;
use hyper::client::conn;
use hyper::body::Body;

pub struct NoBody;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "clients-kube")]
    KubernetesError(k8s::Error),

    ConnectError(String),
    ConnectionError(String),
    // TODO: is this an FSPIOP API error, or a Mojaloop error? I.e. is this type defined in the
    // FSPIOP API spec, or does it exist only in the _Mojaloop_ implementation of that spec?
    MojaloopApiError(fspiox_api::ErrorResponse),
    InvalidResponseBody(String),
    FailureToDeserializeResponseBody(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl<'de> Deserialize<'de> for NoBody {
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<NoBody, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() == 0 {
            Ok(NoBody {})
        } else {
            Err(serde::de::Error::custom(format!("Expected empty response, received {}", s)))
        }
    }
}

pub async fn send<ReqBody, RespBody>(
    sender: &mut conn::SendRequest<Body>,
    msg: ReqBody,
) -> Result<RespBody> where
    ReqBody: serde::Serialize + std::fmt::Debug + Clone,
    RespBody: serde::de::DeserializeOwned,
    http::Request<hyper::Body>: From<ReqBody>,
{
    let resp = sender.send_request(msg.clone().into()).await
        .map_err(|e| Error::ConnectionError(format!("{}", e)))?;

    // Got the response okay, need to check if we have an ML API error
    let (parts, body) = resp.into_parts();

    let body_bytes = hyper::body::to_bytes(body).await
        .map_err(|e| Error::ConnectionError(format!("{}", e)))?;

    if parts.status.is_success() {
        serde_json::from_slice::<RespBody>(&body_bytes).map_err(|e|
            Error::FailureToDeserializeResponseBody(
                format!(
                    "Failed to deserialize FSPIOP response from request {:?}. Error: {}. Body: {}.",
                    msg,
                    e,
                    std::str::from_utf8(&body_bytes).unwrap(),
                )
            )
        )
    } else {
        // In case of an HTTP error response (response code > 399), attempt to deserialize a
        // Mojaloop API error from the response.
        serde_json::from_slice::<fspiox_api::ErrorResponse>(&body_bytes)
            .map_or_else(
                |e| Err(Error::InvalidResponseBody(
                        format!(
                            // TODO: is this an FSPIOP API error, or a Mojaloop error?
                            "Unhandled error parsing FSPIOP API error out of response {} {}",
                            std::str::from_utf8(&body_bytes).unwrap(),
                            e,
                        )
                    )
                ),
                |ml_api_err| Err(Error::MojaloopApiError(ml_api_err))
            )
    }
}
