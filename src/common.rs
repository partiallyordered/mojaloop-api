pub enum Method {
    GET,
    POST,
    PUT,
}

// TODO: the trait bound on serde::Serialize should probably be behind a feature
pub trait CentralLedgerRequest<T: serde::Serialize> {
    // TODO: a constant function to parse a path and validate it. This means any errors specifying
    // a valid path can be caught at compile time instead of runtime. Look at the implementation of
    // http::uri::PathAndQuery. See: https://doc.rust-lang.org/reference/const_eval.html
    fn path(&self) -> String;
    // TODO: GET requests don't have a body, should/can we make the body optional? Do we need
    // different types, i.e. CentralLedgerRequestWithBody or CentralLedgerPostRequest?
    fn body(&self) -> T;
    const METHOD: Method;
    const CONTENT_TYPE: &'static str = "application/json";
    const ACCEPT: &'static str = "application/json";
}
