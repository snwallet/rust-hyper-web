#![deny(warnings)]

// use futures_util::TryStreamExt;
use hyper::{Body, Request, Response, StatusCode, header};

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use url::form_urlencoded;

#[derive(Serialize, Deserialize)]
struct JsonResult<T> {
    errno:i8,
    errmsg:String,
    data:T
}

pub fn nofound() -> Result<Response<Body>, hyper::Error>{
    static NOTFOUND: &[u8] = b"404";
    Ok(Response::builder().status(StatusCode::NOT_FOUND).body(NOTFOUND.into()).unwrap())
}

pub async fn test_post(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let b = hyper::body::to_bytes(req).await?;
    let params = form_urlencoded::parse(b.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    // println!("{:?}",params);

    let res = JsonResult{
        errno: 0,
        errmsg: "".to_string(),
        data:params
    };
    let json = serde_json::to_string(&res).unwrap();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json)).unwrap();
    Ok(response)
}