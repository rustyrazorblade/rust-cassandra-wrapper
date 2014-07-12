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


#[path = "../cassandra/mod.rs"] mod cassandra
{
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

fn print_error(future:CassFuture) {
  let message = future.error_message();
//println!("Error: {}", message.cass_string);//FIXME stderr
}

pub fn create_cluster99() -> CassCluster {unsafe{
  let contact_points = ["127.0.0.1".to_string()];
  let cluster = CassCluster::new();
  for contact_point in contact_points.iter() {
    let cpoint = contact_point.to_c_str();
    cass_internal_api::cass_cluster_setopt( cluster.cass_cluster, CASS_OPTION_CONTACT_POINTS, cpoint.as_ptr() as *const libc::c_void,cpoint.len() as  u64);
  }
  CassCluster{cass_cluster:cluster.cass_cluster}
}}

pub fn execute_query(session:CassSession, query:CString) -> CassError {
  let initted_string = CassValue::string_init(query);
  let statement:CassStatement = CassStatement::new(initted_string, 0, CASS_CONSISTENCY_ONE);
  let future:CassFuture = CassFuture{cass_future:session.execute(statement).cass_future};
  future.wait();
  let rc = future.error_code();
  if rc.cass_error != cassandra::error::CASS_OK {
    print_error(future);
  }
  future.free();
  statement.free();
  return rc;
}

pub fn insert_into_basic(session:CassSession, key:String, basic:Basic) -> CassError {
  let rawString = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
  let query:CassString = CassValue::string_init(rawString.to_string().to_c_str());
  let mut statement = CassStatement::new(query, 6, CASS_CONSISTENCY_ONE);
  statement.bind_string(0, CassValue::string_init(key.to_c_str()));
  statement.bind_bool(1, basic.bln as u32);
  statement.bind_float(2, basic.flt);
  statement.bind_double(3, basic.dbl);
  statement.bind_int32(4, basic.i32);
  statement.bind_int64(5, basic.i64);
  let future:CassFuture = session.execute(statement);
  future.wait();
  let rc:CassError = future.error_code();
  if rc.cass_error != cassandra::error::CASS_OK {
   print_error(future);
  }
  future.free();
  statement.free();
  return rc;
}

pub fn select_from_basic(session:CassSession, key:String, basic:Basic) -> CassError {unsafe{
  let rawString = "SELECT * FROM examples.basic WHERE key = ?;";
  let query = CassValue::string_init(rawString.to_c_str());
  let mut statement = CassStatement::new(query, 1, cass_internal_api::CASS_CONSISTENCY_ONE);
  statement.bind_string(0, CassValue::string_init(key.to_c_str()));
  let future = session.execute(statement);
  future.wait();
  let rc = future.error_code();
  if rc.cass_error != CASS_OK {
    print_error(future);
    return rc;
  } else {
      let result = future.get_result();
      let iterator = CassIterator{cass_iterator:result.iterator()};
      if iterator.next() {
        let row: CassRow = iterator.get_row();
        row.get_column(1).get_bool(if basic.bln {1} else {0});
        row.get_column(2).get_double(basic.dbl);
        row.get_column(3).get_float( basic.flt);
        row.get_column(4).get_int32( basic.i32);
        row.get_column(5).get_int64( basic.i64);
     }
   result.free();
   iterator.free();
 }
  future.free();
  statement.free();
  return rc;
}}

pub fn create_cluster() -> CassCluster {unsafe{
  let contact_points = ["127.0.0.1"];
  let cluster:*mut cass_internal_api::CassCluster = cass_internal_api::cass_cluster_new();
  for contact_point in contact_points.iter() {
    let  c:&'static str=*contact_point;
    let cstr=c.to_c_str();
    cass_internal_api::cass_cluster_setopt( cluster, cass_internal_api::CASS_OPTION_CONTACT_POINTS,  cstr.as_ptr() as *const libc::c_void, cstr.len() as  u64);
  }
  return CassCluster{cass_cluster:cluster}
}}

pub fn connect_session(cluster_:CassCluster)-> (CassError,CassSession) {unsafe{
  let future: CassFuture = CassFuture{cass_future:cass_internal_api::cass_cluster_connect(cluster_.cass_cluster)};
  future.wait();
  let rc = future.error_code();
  let session = CassSession{cass_session:cass_internal_api::cass_future_get_session(future.cass_future)};

  future.free();
  return (rc,session);
}}

fn main()  {
  let cluster = create_cluster();

  let input:Basic = Basic{bln:cass_internal_api::cass_false > 0, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
  let output:Basic=  Basic{bln:false, dbl:0.0f64,flt:0.0f32, i32:0, i64:0};

  let (rc,session) = connect_session(cluster);
  if rc.cass_error != cass_internal_api::CASS_OK {
    // println!("rc={}",rc);
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
  //println!("select and insert matched");
}
