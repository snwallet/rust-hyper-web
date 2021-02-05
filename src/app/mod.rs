#![deny(warnings)]

mod controller;
mod model;
mod lib;

use crate::app::controller::*;

use std::{net::SocketAddr};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};



async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/test") => test_controller::main(req).await,
        (&Method::POST, "/param") => param_controller::main(req).await,
        (&Method::GET, "/file")  => file_controller::main(req).await,
        _ => Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("".to_string())).unwrap()),
    }
}

pub async fn run(port:u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(make_svc);
    println!("server run at http://127.0.0.1:{}",port);
    if let Err(e) = server.await { eprintln!("server error: {}", e); }
}