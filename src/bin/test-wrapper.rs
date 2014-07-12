#![crate_name="test-wrapper"]
#![feature(phase)]
#![feature(globs)]

extern crate libc;

use cassandra::*;
use cassandra::future::*;
//use cassandra::statement::CassStatement;
use cassandra::cluster::*;
use cassandra::statement::*;
use cassandra::session::*;
use cassandra::iterator::*;
use cassandra::error::*;



use std::c_str::CString;

#[path = "../cassandra/mod.rs"] mod cassandra
{
  #[path="../statement.rs"] pub mod statment;
  #[path="../cluster.rs"] pub mod cluster;
  #[path="../option.rs"] pub mod option;
  #[path="../future.rs"] pub mod future;
  #[path="../error.rs"] pub mod error;
  #[path="../session.rs"] pub mod session;
  #[path="../iterator.rs"] pub mod session;
}

pub struct Basic {
  pub bln: bool,
  pub flt: f32,
  pub dbl: f64,
  pub i32: i32,
  pub i64: i64
}

fn print_error(future:*mut cassandra::future::CassFuture) {unsafe{
  let message = future.error_message();
  println!("Error: {}", message.data);//FIXME stderr
}}

pub fn create_cluster() -> *mut cassandra::cluster::CassCluster {unsafe{
  let contact_points = ["127.0.0.1"];
  let cluster:*mut CassCluster =CassCluster::new();
  for contact_point in contact_points.iter() {
    let  c:&'static str=*contact_point;
    let cstr=c.to_c_str();
  //  cass_cluster_setopt( cluster, option::CASS_OPTION_CONTACT_POINTS,  cstr.as_ptr() as *const libc::c_void, cstr.len() as  u64);
  }
  return cluster;
}}

pub fn connect_session(cluster:*mut CassCluster)-> (error::CassError,*mut CassSession) {unsafe{
  let future:*mut CassFuture = cluster.cass_cluster_connect();
  future.wait();
  let rc:CassError = future.error_code();
  let session:*mut CassSession = future.get_session();

  future.free();

  return (rc,session);
}}

pub fn execute_query(session:*mut CassSession, query:CString) -> CassError {unsafe{
  let statement:*mut CassStatement = cass_statement_new(cass_string_init(query.as_ptr() as *const i8), 0, CASS_CONSISTENCY_ONE);

  let future:*mut CassFuture = cass_session_execute(session, statement);
  cass_future_wait(future);

  let rc:CassError = cass_future_error_code(future);
  if rc != CASS_OK {
    print_error(future);
  }

  cass_future_free(future);
  cass_statement_free(statement);

  return rc;
}}

pub fn insert_into_basic(session:*mut CassSession, key:CString, basic:*mut Basic) -> CassError {unsafe{
  let rawString = "INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);";
  let query:CassString = cass_string_init(rawString.as_ptr() as *const i8);
  let statement:*mut CassStatement = cass_statement_new(query, 6, CASS_CONSISTENCY_ONE);
  cass_statement_bind_string(statement, 0, cass_string_init(key.as_ptr() as *const i8));
  cass_statement_bind_bool(statement, 1, (*basic).bln as u32);
  cass_statement_bind_float(statement, 2, (*basic).flt);
  cass_statement_bind_double(statement, 3, (*basic).dbl);
  cass_statement_bind_int32(statement, 4, (*basic).i32);
  cass_statement_bind_int64(statement, 5, (*basic).i64);
  let future:*mut CassFuture = cass_session_execute(session, statement);
  cass_future_wait(future);
  let rc:CassError = cass_future_error_code(future);
  if rc != CASS_OK {
    print_error(future);
  }
  cass_future_free(future);
  cass_statement_free(statement);
  return rc;
}}

pub fn select_from_basic(session:*mut CassSession, key:CString, basic:*mut Basic) -> CassError {unsafe{
  let rawString = "SELECT * FROM examples.basic WHERE key = ?;";
  let query:CassString = cass_string_init(rawString.as_ptr() as *const i8);
  let statement:*mut CassStatement = cass_statement_new(query, 1, CASS_CONSISTENCY_ONE);
  cass_statement_bind_string(statement, 0, cass_string_init(key.as_ptr() as *const i8));
  let future:*mut CassFuture = cass_session_execute(session, statement);
  cass_future_wait(future);
  let rc:CassError = cass_future_error_code(future);
  if rc != CASS_OK {
    print_error(future);
    return 1;
  } else {
      let result:*const CassResult = cass_future_get_result(future);
      let iterator:*mut CassIterator = cass_iterator_from_result(result);
      if cass_iterator_next(iterator) > 0 {
        let row: *const CassRow = cass_iterator_get_row(iterator);
        cass_value_get_bool(cass_row_get_column(row, 1), &mut (if (*basic).bln {1} else {0}))  ;
        cass_value_get_double(cass_row_get_column(row, 2), &mut(*basic).dbl);
        cass_value_get_float(cass_row_get_column(row, 3), &mut (*basic).flt);
        cass_value_get_int32(cass_row_get_column(row, 4), &mut (*basic).i32);
        cass_value_get_int64(cass_row_get_column(row, 5), &mut (*basic).i64);
      }
   cass_result_free(result);
   cass_iterator_free(iterator);
 }
   cass_future_free(future);
  cass_statement_free(statement);
  return rc;
}}

fn main()  {
  let cluster:*mut CassCluster = create_cluster();
  let mut input:Basic = Basic{bln:cass_false > 0, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
  let mut output:Basic=  Basic{bln:false, dbl:0.0f64,flt:0.0f32, i32:0, i64:0};
  let (rc,session) = connect_session(cluster);
  if rc != CASS_OK {
//    println!("rc={}",rc);
    return
  }
  let rawString = "CREATE KEYSPACE if not exists examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
  execute_query(session, rawString.to_c_str());
  let create_table_string = "CREATE TABLE if not exists examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";
  execute_query(session, create_table_string.to_c_str());
  insert_into_basic(session, "test".to_c_str(), &mut input);
  select_from_basic(session, "test".to_c_str(), &mut output);
  println!("Boolean check: {}--{}",input.bln,output.bln);
  assert!(input.flt == output.flt);
  assert!(input.dbl == output.dbl);
  assert!(input.i32 == output.i32);
  assert!(input.i64 == output.i64);
  assert!(input.bln == output.bln);
  println!("select and insert matched");
}
