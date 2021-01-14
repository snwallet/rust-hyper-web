
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};

use super::super::db_conn;

#[derive(Debug, PartialEq, Eq,Serialize, Deserialize)]
pub struct App {
    id: i32,
    title:String,
    url:String,
    logo:String,
}



// pub fn insert() {
//     let mut conn = db_conn();
//
//     let ret = "insert into user(name,pwd)values(?,?)".with((1,2)).run(&mut conn).unwrap();
//     println!("insert id:{:?}",ret.last_insert_id().unwrap());
// }

pub fn select()->Vec<App>{
    let mut conn = db_conn();

    let ret = "select id,title,url,logo from app where id>1 and is_on=1 and is_delete=0".map(&mut conn,|(id,title,url,logo)|{
        App{
            id,
            title,
            url,
            logo
        }
    }).unwrap();
    println!("{:?}",ret);
    ret
}
