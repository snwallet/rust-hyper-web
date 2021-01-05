use futures_util::TryStreamExt;
//use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, HeaderMap};

use super::super::model::help_model;
use hyper::header::CONTENT_TYPE;


pub async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json; charset=utf-8".parse().unwrap());
    // 通过req.method()和req.uri().path()来识别方法和请求路径
    match (req.method(), req.uri().path()) {
        // (&Method::GET, "/") => {
        //     *response.body_mut() = Body::from("Try POSTing data to /echo");
        // },
        // (&Method::POST, "/test") => {
        //     // 将POST的内容保持不变返回
        //     // println!("{:?}",&decode(str_1).unwrap()[..]);
        //     println!("{:?}",req);
        //     let _info1 = Info{id:1,name:"123".to_string()};
        //     let info_str = serde_json::to_string(&_info1).unwrap();
        //     *response.body_mut() = Body::from(info_str);
        // },
        (&Method::POST, "/get_token") => {
            // 将POST的内容保持不变返回
            println!("{:?}",req);
            // *response.headers_mut() =
            *response.body_mut() = Body::from(help_model::get_token());
        },
        (&Method::POST, "/echo/uppercase") => {
            // 把请求stream中的字母都变成大写，并返回
            let mapping = req
                .into_body()
                .map_ok(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });

            // 把stream变成body
            *response.body_mut() = Body::wrap_stream(mapping);
        },
        (&Method::POST, "/echo/reverse") => {
            // 这里需要完整的body，所以需要等待全部的stream并把它们变为bytes
            let full_body = hyper::body::to_bytes(req.into_body()).await?;

            // 把body逆向
            let reversed = full_body.iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();

            *response.body_mut() = reversed.into();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = Body::from("404");
        },
    };


    Ok(response)
}