use serde::{Serialize, Deserialize};

use super::super::model::crypt_model;

extern crate crypto;

// use rand::{ Rng, OsRng };

use std::str;

use base64::{encode, decode};

use time;

const KEY: [u8; 32] = [1, 156, 229, 143, 201, 63, 74, 150, 54, 135, 46, 232, 67, 173, 248, 206, 144, 15, 26, 19, 169, 86, 199, 47, 91, 228, 157, 196, 124, 248, 62, 180];
const IV: [u8; 16] = [128, 129, 20, 181, 129, 136, 32, 201, 227, 149, 3, 131, 69, 245, 49, 177];


#[derive(Serialize, Deserialize)]
pub struct JsonResult<T> {
    errno:u8,
    errmsg:String,
    data:T
}

#[derive(Serialize, Deserialize)]
pub struct AuthToken {
    timestr:String,
    token:String,
}

//get now timestamp
pub fn get_time()->String{
    let timespec = time::get_time();
    let timestr = (timespec.sec + 600) * 1000 + (timespec.nsec as f64 / 1000.0 / 1000.0) as i64;
    timestr.to_string()
}

//encrypt token
pub fn get_token()->String{
    let message = get_time();
    println!("{}",message);
    let encrypted_data = crypt_model::encrypt(message.as_bytes(), &KEY, &IV).ok().unwrap();
    let test_vec = crypt_model::encrypt(message.as_bytes(), &KEY, &IV).ok().unwrap();
    println!("{}",parse_token(encode(&test_vec)));
    println!("{}",check_token(encode(&test_vec)));
    let res = JsonResult{errno:0,errmsg:"success".to_string(), data:AuthToken{timestr:message,token:encode(encrypted_data)} };
    serde_json::to_string(&res).unwrap()
}

//decrypt token
pub fn parse_token(str:String)->String{
    let vec_u8 = &decode(str).unwrap()[..];
    let decrypted_data =  crypt_model::decrypt(&vec_u8[..], &KEY, &IV).ok().unwrap();
    String::from_utf8(decrypted_data).unwrap()
}

//check token is out time or not
pub fn check_token(str:String)->bool{
    let time = parse_token(str).parse::<i64>().unwrap();
    let now = get_time().parse::<i64>().unwrap();
    if time <= now  {true}else{false}
}