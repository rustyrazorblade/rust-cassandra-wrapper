#![feature(globs)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate cass_internal_api;
extern crate libc;

use cassandra::future::*;
use cassandra::cluster::*;
use cassandra::statement::*;
use cassandra::session::*;
use cassandra::types::*;
use cassandra::error::*;
use std::c_str::CString;

use cassandra::iterator::CassIterator;
use cassandra::row::CassRow;
use cassandra::result::CassResult;

use cassandra::cluster::CASS_OPTION_CONTACT_POINTS;
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
  #[path="../session.rs"] pub mod session;
}

pub struct Basic {
  pub bln: bool,
  pub flt: f32,
  pub dbl: f64,
  pub i32: i32,
  pub i64: i64
}

fn print_error(future:&mut CassFuture) {
  println!("Error: {}", future);
}


pub fn execute_query(session:CassSession, query:CString) -> CassError {
  let inited_string = CassValue::string_init(query);
  let mut statement = CassStatement::new(inited_string, 0, CASS_CONSISTENCY_ONE);
  let mut future:&mut CassFuture = &mut CassFuture{cass_future:session.execute(statement).cass_future};
  future.wait();
  let rc = future.error_code();
  if rc.cass_error != cassandra::error::CASS_OK {
    print_error(future);
  }
  return future.error_code();
}

pub fn insert_into_basic(session:CassSession, key:String, basic:Basic) -> CassError {
  let rawString = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
  let query = CassValue::string_init(rawString.to_string().to_c_str());
  let mut statement = CassStatement::new(query, 6, CASS_CONSISTENCY_ONE);
  statement.bind_string(0, CassValue::string_init(key.to_c_str()));
  statement.bind_bool(1, basic.bln as u32);
  statement.bind_float(2, basic.flt);
  statement.bind_double(3, basic.dbl);
  statement.bind_int32(4, basic.i32);
  statement.bind_int64(5, basic.i64);
  let mut future = &mut session.execute(statement);
  future.wait();
  let rc = future.error_code();
  if rc.cass_error != cassandra::error::CASS_OK {
   print_error(future);
  }
  return rc;
}

pub fn select_from_basic(session:CassSession, key:String, basic:Basic) -> CassError {
  let rawString = "SELECT * FROM examples.basic WHERE key = ?;";
  let query = CassValue::string_init(rawString.to_c_str());
  let mut statement = CassStatement::new(query, 1, CASS_CONSISTENCY_ONE);
  statement.bind_string(0, CassValue::string_init(key.to_c_str()));
  let mut future = session.execute(statement);
  future.wait();
  let rc = future.error_code();
  if rc.cass_error != CASS_OK {
    print_error(&mut future);
    return rc;
  } else {
      let result = future.get_result();
      let mut iterator = &mut CassIterator{cass_iterator:result.iterator()};
      if iterator.next() {
        let row: CassRow = iterator.get_row();
        row.get_column(1).get_bool(if basic.bln {1} else {0});
        row.get_column(2).get_double(basic.dbl);
        row.get_column(3).get_float( basic.flt);
        row.get_column(4).get_int32( basic.i32);
        row.get_column(5).get_int64( basic.i64);
     }
 }
  return rc;
}

pub fn create_cluster() -> CassCluster {
  let contact_points = ["127.0.0.1".to_string()];
  let mut cluster = CassCluster::new();
  for contact_point in contact_points.iter() {
    let cpoint = contact_point.to_c_str();
    cluster.setopt(CASS_OPTION_CONTACT_POINTS, cpoint);
  }
  cluster
}

pub fn connect_session(mut cluster:CassCluster) -> (CassError,CassSession) {
  let mut future: CassFuture = cluster.connect();
  future.wait();
  let rc = future.error_code();
  let session = future.get_session();

  return (rc,session);
}

fn main()  {
  let cluster = create_cluster();

  let input:Basic = Basic{bln:false, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
  let output:Basic=  Basic{bln:false, dbl:0.0f64,flt:0.0f32, i32:0, i64:0};

  let (rc,session) = connect_session(cluster);
  if rc.cass_error != cass_internal_api::CASS_OK {
    println!("rc={}",rc.cass_error);
  return
  }

  insert_into_basic(session, "test2".to_string(), input);
  select_from_basic(session, "test".to_string(), output);
  println!("Boolean check: {}--{}",input.bln,output.bln);
  assert!(input.flt == output.flt);
  assert!(input.dbl == output.dbl);
  assert!(input.i32 == output.i32);
  assert!(input.i64 == output.i64);
  assert!(input.bln == output.bln);
  println!("select and insert matched");
}
