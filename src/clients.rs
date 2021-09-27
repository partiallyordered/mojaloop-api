pub mod central_ledger;

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
