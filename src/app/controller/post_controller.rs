
// use futures_util::TryStreamExt;
use hyper::{Body, Request, Response, StatusCode, header};

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use url::form_urlencoded;

#[derive(Serialize, Deserialize)]
struct JsonResult<T> { errmsg:String, data:T, errno:i8 }




fn string_response(str:String)->Result<Response<Body>, hyper::Error>{
    Ok(Response::builder()
           .status(StatusCode::OK)
           .header(header::CONTENT_TYPE, "application/json")
           .body(Body::from(str)).unwrap())
}


pub fn nodata_response(number:i8,msg:String)->Result<Response<Body>, hyper::Error>{
    let res =  JsonResult{
        errno:number,
        errmsg: msg,
        data: ""
    };
    let json = serde_json::to_string(&res).unwrap();
    Ok(Response::builder()
           .status(StatusCode::OK)
           .header(header::CONTENT_TYPE, "application/json")
           .body(Body::from(json)).unwrap())
}



// 404
pub fn nofound() -> Result<Response<Body>, hyper::Error>{
    nodata_response(-1,"404".to_string())
}

//post param as json
pub async fn test_post(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let b = hyper::body::to_bytes(req).await?;
    let params = form_urlencoded::parse(b.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    println!("{:?}",params);

    let res = JsonResult{
        errno: 0,
        errmsg: "success".to_string(),
        data:params
    };
    let json = serde_json::to_string(&res).unwrap();
    string_response(json)
}

//get a token
pub async fn get_token(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let b = hyper::body::to_bytes(req).await?;
    let params = form_urlencoded::parse(b.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    println!("{:?}",params);
    if let (Some(name),Some(pwd)) = (params.get("name"),params.get("pwd") ) {
        if (name,pwd)==(&"test".to_string(),&"123456".to_string()) {
            let res = JsonResult{
                errno: 0,
                errmsg: "success".to_string(),
                data:crate::app::model::help_model::get_token()
            };
            let json = serde_json::to_string(&res).unwrap();
            string_response(json)
        }else{
            nodata_response(-1,"param error".to_string())
        }
    } else {
         nodata_response(-1,"param error".to_string())
    }

}
