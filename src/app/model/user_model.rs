
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};

// use super::super::model;

#[derive(Debug, PartialEq, Eq,Serialize, Deserialize)]
pub struct User {
    id: i32,
    name:String,
}

fn db_conn() -> PooledConn {
    let dsn = String::from("mysql://root:root@127.0.0.1:3306/test");
    let pool = Pool::new(dsn).unwrap();
    pool.get_conn().unwrap()
}

// pub fn insert() {
//     let mut conn = db_conn();
//
//     let ret = "insert into user(name,pwd)values(?,?)".with((1,2)).run(&mut conn).unwrap();
//     println!("insert id:{:?}",ret.last_insert_id().unwrap());
// }

pub fn select()->Vec<User>{
    let mut conn = db_conn();

    let ret = "select id,name from user".map(&mut conn,|(id,name)|{
        User{
            id:id,
            name:name,
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