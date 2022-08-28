use hyper::{Body, Request, Response};

use crate::response_util::not_found;

use super::Route;

pub struct NotFound {}

impl NotFound {
    pub fn new() -> Self {
        Self {}
    }
}

impl Route for NotFound {
    fn method(&self) -> String {
        "GET".into()
    }

    fn path(&self) -> String {
        "/404".into()
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        not_found()
    }
}
