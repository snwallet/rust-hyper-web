
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};

use super::super::db_conn;

#[derive(Debug, PartialEq, Eq,Serialize, Deserialize)]
pub struct User {
    id: i32,
    wallet:String,
}



pub fn _insert(address:String, ip:String) -> u64 {
    let mut conn = db_conn();

    let ret = "insert into user(wallet,first_id)values(?,?)".with((address,ip)).run(&mut conn).unwrap();
    println!("insert id:{:?}",ret.last_insert_id().unwrap());
    ret.last_insert_id().unwrap()
}

pub fn _select()->Vec<User>{
    let mut conn = db_conn();

    let ret = "select id,wallet from user".map(&mut conn,|(id,wallet)|{
        User{
            id,
            wallet,
        }
    }).unwrap();
    println!("{:?}",ret);
    ret
}
//
// pub fn update() {
//     let mut conn = db_conn();
//
//
//     let ret = "update `user` set name=? WHERE id=?".with(("test5",5)).run(&mut conn).unwrap();
//     println!("affect row:{:?}",ret.affected_rows());
// }
//
// pub fn delete() {
//     let mut conn = db_conn();
//
//
//     let ret = "DELETE FROM `user` WHERE id=? and ?".with((1,true)).run(&mut conn).unwrap();
//     println!("affect row:{:?}",ret.affected_rows());
// }