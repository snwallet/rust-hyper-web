#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Server};
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let addr = ([127, 0, 0, 1], 8888).into();

    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(app::router))
    }));

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}