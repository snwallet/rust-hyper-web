#![deny(warnings)]

mod controller;

mod model;

use hyper::{Body, Method, Request, Response,};

use crate::app::controller::post_controller::*;
use crate::app::model::*;

use base64::{decode};

// use mysql::{PooledConn, Pool};

// pub fn db_conn() -> PooledConn {
//     let dsn = String::from("mysql://root:root@192.168.0.123:3306/carpark");
//     let pool = Pool::new(dsn).unwrap();
//     pool.get_conn().unwrap()
// }


pub async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    if req.uri() == "/get_token" {
        return get_token(req).await;
    }else{
        if let Some(token) = req.headers().get("token") {
            if help_model::check_token(&decode(token).unwrap()[..]) {
                
            }else{
               return nofound();
            }
        }
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/test") => test_post(req).await,
            _ => nofound(),
        }
    }
}