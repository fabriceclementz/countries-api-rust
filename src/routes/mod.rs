pub mod countries;
pub mod country;
pub mod not_found;

use hyper::{Body, Request, Response};

pub trait Route {
    fn method(&self) -> String;
    fn path(&self) -> String;
    fn handle(&self, request: Request<Body>) -> Response<Body>;
}
