#![deny(warnings)]

use hyper::{Body, Method, Request, Response,};
use super::super::controller::post_controller::*;

pub async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    match (req.method(), req.uri().path()) {

        (&Method::POST, "/test") => test_post(req).await,
        (&Method::POST, "/get_token") => test_post(req).await,

        _ => nofound(),
    }
}



