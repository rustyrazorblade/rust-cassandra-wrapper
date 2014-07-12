#![feature(globs)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate cass_internal_api;
extern crate libc;
//use cass_internal_api::*;
//cassandra::cluster::CassCluster::

//use cassandra::*;
use cassandra::future::*;
use cassandra::cluster::*;
use cassandra::statement::*;
use cassandra::session::*;
use cassandra::types::*;
use cassandra::error::*;



use std::c_str::CString;

#[path = "../cassandra/mod.rs"] mod cassandra
{
#[path="../statement.rs"] pub mod statement;
#[path="../types.rs"] pub mod types;
  #[path="../cluster.rs"] pub mod cluster;
  // #[path="../option.rs"] pub mod option;
  #[path="../future.rs"] pub mod future;
  #[path="../error.rs"] pub mod error;
  #[path="../session.rs"] pub mod session;
  // #[path="../iterator.rs"] pub mod iterator;
}

pub struct Basic {
  pub bln: bool,
  pub flt: f32,
  pub dbl: f64,
  pub i32: i32,
  pub i64: i64
}

fn print_error(future:CassFuture) {unsafe{
  let message = future.error_message();
  println!("Error: {}", message.data);//FIXME stderr
}}

pub fn create_cluster99() -> cass_internal_api::CassCluster {unsafe{
  let contact_points = ["127.0.0.1".to_string()];
  let cluster:CassCluster = CassCluster{cass_cluster:&mut*cass_internal_api::cass_cluster_new()};
  for contact_point in contact_points.iter() {
    let cpoint = contact_point.to_c_str();
    cass_internal_api::cass_cluster_setopt( &mut*cluster.cass_cluster, cass_internal_api::CASS_OPTION_CONTACT_POINTS, cpoint.as_ptr() as *const libc::c_void,cpoint.len() as  u64);
  }
  return *cluster.cass_cluster;
}}

pub fn execute_query(session:CassSession, query:CString) -> CassError {unsafe{
  let initted_string = cassandra::types::string_init(query);
  let statement:CassStatement = CassStatement{cass_statement:
          cass_internal_api::cass_statement_new(initted_string, 0, cass_internal_api::CASS_CONSISTENCY_ONE)
  };
  let future:CassFuture = CassFuture{cass_future:cass_internal_api::cass_session_execute(session.cass_session, statement.cass_statement)};
  cass_internal_api::cass_future_wait(future.cass_future);

  let rc = future.error_code();
  if rc.cass_error != cassandra::error::CASS_OK {
    print_error(future);
  }

  future.free();
  statement.free();

  return rc;
}}

pub fn insert_into_basic(session:*mut cass_internal_api::CassSession, key:CString, basic:*mut Basic) -> cass_internal_api::CassError {unsafe{
  let rawString = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
  let query:cass_internal_api::CassString = cass_internal_api::cass_string_init(rawString.as_ptr() as *const i8);
  let statement = CassStatement{cass_statement:cass_internal_api::cass_statement_new(query, 6, cass_internal_api::CASS_CONSISTENCY_ONE)};
  cass_internal_api::cass_statement_bind_string(statement.cass_statement, 0, cass_internal_api::cass_string_init(key.as_ptr() as *const i8));
  cass_internal_api::cass_statement_bind_bool(statement.cass_statement, 1, (*basic).bln as u32);
  cass_internal_api::cass_statement_bind_float(statement.cass_statement, 2, (*basic).flt);
  cass_internal_api::cass_statement_bind_double(statement.cass_statement, 3, (*basic).dbl);
  cass_internal_api::cass_statement_bind_int32(statement.cass_statement, 4, (*basic).i32);
  cass_internal_api::cass_statement_bind_int64(statement.cass_statement, 5, (*basic).i64);
  let future:CassFuture = CassSession{cass_session:session}.execute(statement);
  cass_internal_api::cass_future_wait(future.cass_future);
  let rc:cass_internal_api::CassError = cass_internal_api::cass_future_error_code(future.cass_future);
  if rc != cassandra::error::CASS_OK {
    print_error(future);
  }
  cass_internal_api::cass_future_free(future.cass_future);
  cass_internal_api::cass_statement_free(statement.cass_statement);
  return rc;
}}

pub fn select_from_basic(session:CassSession, key:CString, basic:*mut Basic) -> cass_internal_api::CassError {unsafe{
  let rawString = "SELECT * FROM examples.basic WHERE key = ?;";
  let query:cass_internal_api::CassString = cass_internal_api::cass_string_init(rawString.as_ptr() as *const i8);
  let statement:*mut cass_internal_api::CassStatement = cass_internal_api::cass_statement_new(query, 1, cass_internal_api::CASS_CONSISTENCY_ONE);
  cass_internal_api::cass_statement_bind_string(statement, 0, cass_internal_api::cass_string_init(key.as_ptr() as *const i8));
  let future:*mut cass_internal_api::CassFuture = cass_internal_api::cass_session_execute(session.cass_session, statement);
  cass_internal_api::cass_future_wait(future);
  let rc:cass_internal_api::CassError = cass_internal_api::cass_future_error_code(future);
  if rc != CASS_OK {
    print_error(CassFuture{cass_future:future});
    return 1;
  } else {
      let result:*const cass_internal_api::CassResult = cass_internal_api::cass_future_get_result(future);
      let iterator:*mut cass_internal_api::CassIterator = cass_internal_api::cass_iterator_from_result(result);
      if cass_internal_api::cass_iterator_next(iterator) > 0 {
        let row: *const cass_internal_api::CassRow = cass_internal_api::cass_iterator_get_row(iterator);
        cass_internal_api::cass_value_get_bool(cass_internal_api::cass_row_get_column(row, 1), &mut (if (*basic).bln {1} else {0}))  ;
        cass_internal_api::cass_value_get_double(cass_internal_api::cass_row_get_column(row, 2), &mut(*basic).dbl);
        cass_internal_api::cass_value_get_float(cass_internal_api::cass_row_get_column(row, 3), &mut (*basic).flt);
        cass_internal_api::cass_value_get_int32(cass_internal_api::cass_row_get_column(row, 4), &mut (*basic).i32);
        cass_internal_api::cass_value_get_int64(cass_internal_api::cass_row_get_column(row, 5), &mut (*basic).i64);
      }
   cass_internal_api::cass_result_free(result);
   cass_internal_api::cass_iterator_free(iterator);
 }
  cass_internal_api::cass_future_free(future);
  cass_internal_api::cass_statement_free(statement);
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
//  let cluster_:*mut cass_internal_api::CassCluster = create_cluster2();
  let cluster = create_cluster();

//  let *mut cluster:cass_internal_api::CassCluster = create_cluster();
  let mut input:Basic = Basic{bln:cass_internal_api::cass_false > 0, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
  let mut output:Basic=  Basic{bln:false, dbl:0.0f64,flt:0.0f32, i32:0, i64:0};

//THIS LINE CAUSES EXECUTION TO GO WRONG
//println!("cluster:{}","foo");
  let (rc,session) = connect_session(cluster);
  if rc.cass_error != cass_internal_api::CASS_OK {
//    println!("rc={}",rc);
    return
  }

  let rawString = "CREATE KEYSPACE if not exists examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };".to_c_str();
  execute_query(session, rawString);
  let create_table_string = "CREATE TABLE if not exists examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));".to_c_str();
  execute_query(session, create_table_string);
  insert_into_basic(session.cass_session, "test".to_c_str(), &mut input);
  select_from_basic(session, "test".to_c_str(), &mut output);
  println!("Boolean check: {}--{}",input.bln,output.bln);
  assert!(input.flt == output.flt);
  assert!(input.dbl == output.dbl);
  assert!(input.i32 == output.i32);
  assert!(input.i64 == output.i64);
  assert!(input.bln == output.bln);
  println!("select and insert matched");
}
