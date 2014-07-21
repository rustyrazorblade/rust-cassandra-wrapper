#![feature(globs)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate cass_internal_api;
extern crate libc;


use std::c_str::CString;

use cassandra::cluster::*;
use cassandra::statement::*;
use cassandra::session::*;
use cassandra::types::*;
use cassandra::error::*;
use cassandra::batch::*;


use cassandra::row::CassRow;
use cassandra::result::CassResult;

use cassandra::consistency::CASS_CONSISTENCY_ONE;

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
  #[path="../batch.rs"] pub mod batch;
  #[path="../session.rs"] pub mod session;
}

#[deriving(Show, Clone)]
pub struct Basic {
  pub bln: bool,
  pub flt: f32,
  pub dbl: f64,
  pub i32: i32,
  pub i64: i64
}



pub fn insert_into_basic(session:&CassSession, key:String, basic:Basic) -> Result<CassResult,CassError> {
  let query_string = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);".to_string();
  let mut statement = CassStatement::build_from_string(query_string, 6, CASS_CONSISTENCY_ONE);
  println!("inserting key:{}",key);
  statement.bind_string(0, CassValue::string_init(&key));
  statement.bind_bool(1, basic.bln as u32);
  statement.bind_float(2, basic.flt);
  statement.bind_double(3, basic.dbl);
  statement.bind_int32(4, basic.i32);
  statement.bind_int64(5, basic.i64);
  session.execute(statement)
}

pub fn select_from_basic(session:&CassSession, key:String) -> Result<CassResult,CassError> {
  let query_string = "SELECT * FROM examples.basic WHERE key = ?;".to_string();
  let mut statement = CassStatement::build_from_string(query_string, 6, CASS_CONSISTENCY_ONE);
  let key_str = CassValue::string_init(&key);
  statement.bind_string(0, key_str);
  let future:Result<CassResult,CassError>=session.execute(statement);
  match future {
    Err(err) => return Err(err),
    Ok(result) => {
      return Ok(result)
    }
  }
}

fn main()  {
  let input = Basic{bln:true, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
  let mut output=  Basic{bln:false, dbl:0.0f64, flt:0.00f32, i32:0, i64:0};

  let contact_points = "127.0.0.1".to_string();
let cluster = CassCluster::create(contact_points);

  let (rc,session) = cluster.connect();
  if rc.is_error() {return}

  let insert = insert_into_basic(&session, "test".to_string(), input);

  match insert {
   Err(fail) => println!("result: {}",fail),
   Ok(results) => {}
   }

  let response = select_from_basic(&session, "test".to_string());

  match response {
   Err(fail) => println!("result: {}",fail),
   Ok(results) => {
     let mut iterator = results.iterator();
     if iterator.next() {
       let row:CassRow = iterator.get_row();
       match row.get_column(1).get_bool() {Err(err) => println!("{}--",err),Ok(col) => output.bln=col}
       match row.get_column(2).get_double() {Err(err) => println!("{}--",err),Ok(col) => output.dbl=col}
       match row.get_column(3).get_float() {Err(err) => println!("{}--",err),Ok(col) => output.flt=col}
       match row.get_column(4).get_int32() {Err(err) => println!("{}--",err),Ok(col) => output.i32=col}
       match row.get_column(5).get_int64() {Err(err) => println!("{}--",err),Ok(col) => output.i64=col}
     }
   }
  }

  println!("input :{}",input);
  println!("output:{}",output);
  assert!(input.dbl == output.dbl);
  assert!(input.i32 == output.i32);
  assert!(input.i64 == output.i64);
  assert!(input.bln == output.bln);
  println!("select and insert matched");
}
