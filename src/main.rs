use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
mod model;
mod controller;

#[tokio::main]
async fn main() {
    model::test_model::print_test_model();
    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc 
    = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(controller::test_controller::echo))
    });

    let server 
    = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}