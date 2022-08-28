use hyper::{Body, Request, Response};

use crate::country::CountryRepository;
use crate::response_util::{internal_server_error, ok};

use super::Route;

pub struct Countries {
    repo: CountryRepository,
}

impl Countries {
    pub fn new(repo: CountryRepository) -> Self {
        Self { repo }
    }
}

impl Route for Countries {
    fn method(&self) -> String {
        "GET".into()
    }

    fn path(&self) -> String {
        "/countries".into()
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let countries = self.repo.all();

        match serde_json::to_string(&countries) {
            Ok(json) => ok(json),
            Err(_) => internal_server_error(),
        }
    }
}
