
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn main(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if let Ok(file) = File::open("src/main.rs").await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }else{
        JsonRes::new(-1,"error".to_string(),"")
    }
}
