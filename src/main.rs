use std::convert::Infallible;

use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tracing::{error, info};

use countries_api::country::CountryRepository;
use countries_api::error::CountryError;
use countries_api::response_util::internal_server_error;
use countries_api::router::Router;
use countries_api::routes::countries::Countries;
use countries_api::routes::country::Country;
use countries_api::routes::not_found::NotFound;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, CountryError> {
    info!("init request handler");

    let repo = match CountryRepository::new() {
        Ok(repo) => repo,
        Err(err) => {
            error!("error while creating the country_repository: {}", err);
            return Ok(internal_server_error());
        }
    };

    let mut router = Router::new();
    router.register(Box::new(NotFound::new()));
    router.register(Box::new(Countries::new(repo.clone())));
    router.register(Box::new(Country::new(repo)));

    let response = router.handle(req);

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("server listening at 127.0.0.1:3000");

    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}
