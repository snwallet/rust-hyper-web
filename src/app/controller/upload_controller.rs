use hyper::{header::CONTENT_TYPE, Body, Request, Response};
use multer::{Multipart, Constraints, SizeLimit};
use std::path::Path;
use std::fs::{DirBuilder, File};
use std::io::Write;
use futures::stream;
use serde::{Serialize};
use crate::app::lib::json_res::JsonRes;
use chrono::Local;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let boundary = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| multer::parse_boundary(ct).ok());
    if boundary.is_none() {
        JsonRes::new(-1, "failed multipart/form-data supports only!".to_string(), "")
    }else {    // parse the multipart from request's body
        let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
        let body_vec: Vec<Result<Vec<u8>, std::io::Error>> = vec![Ok(full_body.to_vec())];
        let constraints = Constraints::new()
            // We only accept `my_text_field` and `my_file_field` fields,
            // For any unknown field, we will throw an error.
            .allowed_fields(vec!["file"])
            .size_limit(
                SizeLimit::new()
                    // Set 15mb as size limit for the whole stream body.
                    .whole_stream(10 * 1024 * 1024)
                    // Set 10mb as size limit for all fields.
                    .per_field(10 * 1024 * 1024)
                    // Set 30kb as size limit for our text field only.
                    .for_field("file", 2 * 1024 * 1024),
            );

        let mut multipart = Multipart::new_with_constraints(stream::iter(body_vec), boundary.unwrap(),constraints);

        let mut result: Vec<FileObject> = vec![];

        if let Some(mut field) = multipart.next_field().await.expect("constraints error") {
            let name = field.name().unwrap().to_owned();
            let filename = field.file_name().unwrap().to_owned();

            println!("{:?}:{:?}", name, filename);

            let mut content = Vec::new();
            while let Some(chunk) = field.chunk().await.unwrap() {
                content.extend(chunk.to_vec());
            }

            if content.len()>2*1024*1024 {
                JsonRes::new(-2, "upload file limit 2M".to_string(), "")
            }else {
                println!("file size = {:?}", content.len());

                let mut file = FileObject::new(filename, name, "".to_owned());
                match file.save_file(&content) {
                    Ok(_) => {
                        println!("{:?} save done!", file.uri);
                        result.push(file);
                        JsonRes::new(0, "success".to_string(), Some(result))
                    }
                    Err(e) => {
                        println!("something went wrong: {:?}!", e.to_string());
                        file.uri = e.to_string();
                        JsonRes::new(-3, e.to_string(), "")
                    }
                }
            }
        }else{
            JsonRes::new(-2, "failed".to_string(), "")
        }
    }
}

#[derive(Serialize)]
struct FileObject {
    filename: String,
    name: String,
    uri: String,
}

impl FileObject {
    fn new(filename: String, name: String, uri: String) -> Self {
        Self {
            filename,
            name,
            uri,
        }
    }


    fn save_file(&mut self, content: &[u8]) -> Result<(), std::io::Error> {
        let file_ext = Path::new(&self.filename).extension().unwrap().to_str().unwrap().to_lowercase();
        let file_dir = Local::now().format("%Y-%m-%d").to_string();
        let path = Path::new("./src/app/public/upload").join(&file_dir);
        let filename = format!("{}.{}", Local::now().timestamp_millis(),file_ext);
        DirBuilder::new().recursive(true).create(&path).unwrap();
        self.uri =format!("{}/{}/{}", "/public/upload",file_dir,&filename);

        if path.exists() { return Ok(()); }

        match File::create(&path) {
            Ok(mut f) => {
                f.write_all(content)?;
                Ok(())
            }
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::Other, e.to_string());
                return Err(err);
            }
        }
    }
}
