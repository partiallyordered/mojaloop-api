pub enum Method {
    POST,
}

pub trait CentralLedgerRequest {
    // TODO: a constant function to parse a path and validate it. Look at the implementation of
    // http::uri::PathAndQuery. See: https://doc.rust-lang.org/reference/const_eval.html
    const PATH: &'static str;
    const METHOD: Method;
    const CONTENT_TYPE: &'static str = "application/json";
    const ACCEPT: &'static str = "application/json";
}
