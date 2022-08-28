use hyper::{Body, Request, Response};

use crate::country::CountryRepository;
use crate::response_util::{internal_server_error, not_found, ok};

use super::Route;

pub struct Country {
    repo: CountryRepository,
}

impl Country {
    pub fn new(repo: CountryRepository) -> Self {
        Self { repo }
    }
}

impl Route for Country {
    fn method(&self) -> String {
        "GET".into()
    }

    fn path(&self) -> String {
        "/countries/{code}".into()
    }

    fn handle(&self, _request: Request<Body>) -> Response<Body> {
        // TODO: how to get the code
        match self.repo.by_code("FR") {
            Ok(country) => match serde_json::to_string(&country) {
                Ok(json) => ok(json),
                Err(_) => internal_server_error(),
            },
            Err(_) => not_found(),
        }
    }
}
