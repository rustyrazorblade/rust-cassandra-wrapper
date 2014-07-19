#![feature(globs)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate libc;
extern crate collections;
extern crate cass_internal_api;

use cassandra::cluster::*;
use cassandra::statement::*;
use cassandra::types::*;
use cassandra::session::CassSession;
use cassandra::error::CassError;
use std::collections::DList;
use cassandra::future::CassFuture;

use cassandra::result::CassResult;

use std::c_str::CString;

use cassandra::types::CassString;

use collections::Deque;

use cassandra::cluster::CASS_OPTION_CONTACT_POINTS;
use cassandra::consistency::CASS_CONSISTENCY_ONE;


use std::kinds::marker::NoCopy;

#[path = "../cassandra/mod.rs"] mod cassandra {
  #[path="../statement.rs"] pub mod statement;
  #[path="../consistency.rs"] pub mod consistency;
  #[path="../iterator.rs"] pub mod iterator;
  #[path="../collection.rs"] pub mod collection;
  #[path="../types.rs"] pub mod types;
  #[path="../cluster.rs"] pub mod cluster;
  #[path="../result.rs"] pub mod result;
  #[path="../row.rs"] pub mod row;
  #[path="../future.rs"] pub mod future;
  #[path="../error.rs"] pub mod error;
  #[path="../session.rs"] pub mod session;
}

static NUM_CONCURRENT_REQUESTS:uint = 100;

fn execute_query(session:CassSession, query:String) -> CassError {
  let statement = CassStatement::build_from_string(query, 0, CASS_CONSISTENCY_ONE);

  let mut future:CassFuture = session.execute_async(statement);
  future.wait();
  future.error_code()
}

fn insert_into_async(session:CassSession, key:String) {unsafe{
  let query =  "INSERT INTO async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);".to_string();

  let mut futures:DList<CassFuture> = DList::new();

  let mut i:uint = 0;
  while i < NUM_CONCURRENT_REQUESTS {
  let mut statement = CassStatement::build_from_string(query.clone(), 6, CASS_CONSISTENCY_ONE);

  let wrapped = CassValue::string_init(key + i.to_string());
    println!("response:{}",wrapped);


    statement.bind_string(0, wrapped);
    statement.bind_bool(1, if i % 2 == 0 {true} else {false} as u32);
    statement.bind_float(2, i as f32 / 2.0);
    statement.bind_double(3, i  as f64 / 200.0);
    statement.bind_int32(4, (i as i32 * 10));
    statement.bind_int64(5,(i as i64 * 100));
    session.execute(statement);

    i+=1;
  }

  while (i < futures.len()) {
    let mut future = futures.pop_front();
    let rc =future.unwrap().wait();
    i+=1;
  }
}}

fn main() {

  let mut cluster = CassCluster::new();
  let contact_points = ["127.0.0.1".to_string().to_c_str()];

for contact_point in contact_points.iter() {
  cluster.setopt(CASS_OPTION_CONTACT_POINTS, contact_point);
}

  let (rc,session) = cluster.connect();
  //  session_future.wait();
  println!("foo");
  execute_query(session, "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };".to_string());
  execute_query(session, "CREATE TABLE examples.async (
    key text,
    bln boolean,
    flt float, dbl double,
    i32 int, i64 bigint,
    PRIMARY KEY (key));".to_string());

  execute_query(session, "USE examples".to_string());

  insert_into_async(session, "test".to_string());

  let mut close_future = session.close_async();
   close_future.wait();
}
