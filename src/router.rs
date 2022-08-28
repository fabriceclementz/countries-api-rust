use hyper::{Body, Request, Response};
use regex::Regex;
use tracing::info;

use crate::routes::Route;

pub struct Router {
    routes: Vec<Box<dyn Route>>,
}

impl Router {
    pub fn new() -> Self {
        info!("init router");
        Self { routes: vec![] }
    }

    pub fn register(&mut self, route: Box<dyn Route>) {
        info!("registering route: {} {}", route.method(), route.path());
        self.routes.push(route);
    }

    pub fn handle(&self, req: Request<Body>) -> Response<Body> {
        let route = self.route(req.method().as_str(), req.uri().path());
        info!("matching route: {} {}", route.method(), route.path());
        route.handle(req)
    }

    fn route(&self, method: &str, path: &str) -> &dyn Route {
        let route = self.routes.iter().find(|route| {
            let method_matched = route.method() == method;
            if !method_matched {
                return false;
            }

            // Exact match
            if route.path() == path {
                return true;
            }

            // Match pattern
            let re = Regex::new(route.path().as_str()).unwrap();
            let match_path = re.is_match(path);
            match_path
        });

        match route {
            Some(route) => route,
            None => self
                .routes
                .iter()
                .find(|route| route.method() == "GET" && route.path() == "/404")
                .unwrap(),
        }
        .as_ref()
    }
}
