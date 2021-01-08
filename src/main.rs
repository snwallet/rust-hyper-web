#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
mod controller;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8080).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(controller::router_controller::router)) });
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}