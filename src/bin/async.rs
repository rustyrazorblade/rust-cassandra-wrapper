extern crate log;
extern crate libc;
extern crate collections;
extern crate cassandra;

use cassandra::CassCluster;
use cassandra::CassValue;
use cassandra::CassStatement;
use cassandra::CassFuture;
use cassandra::CassSession;
use cassandra::CassError;

use std::collections::DList;

use collections::Deque;

static NUM_CONCURRENT_REQUESTS:uint = 100;

fn execute_query(session:&CassSession, query:String) -> CassError {
  let statement = CassStatement::build_from_string(query, 0);

  let mut future:CassFuture = session.execute_async(&statement);
  future.wait();
  future.error_code()
}

fn insert_into_async(session:&CassSession, key:String) {
  let query =  "INSERT INTO async (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);".to_string();

  let mut futures:DList<CassFuture> = DList::new();

  let mut i:uint = 0;
  while i < NUM_CONCURRENT_REQUESTS {
  let mut statement = CassStatement::build_from_string(query.clone(), 6);

  let wrapped = CassValue::string_init(key + i.to_string());
    println!("response:{}",wrapped);
    statement.bind_string(0, wrapped);
    statement.bind_bool(1, if i % 2 == 0 {true} else {false} as u32);
    statement.bind_float(2, i as f32 / 2.0);
    statement.bind_double(3, i  as f64 / 200.0);
    statement.bind_int32(4, (i as i32 * 10));
    statement.bind_int64(5,(i as i64 * 100));
    session.execute(&statement);

    i+=1;
  }

  while i < futures.len() {
    let future = futures.pop_front();
    let rc =future.unwrap().wait();
    i+=1;
  }
}

fn main() {
  let contact_points = "127.0.0.1".to_string();
  let cluster = CassCluster::create(contact_points);

// for contact_point in contact_points.iter() {
//   cluster.setopt(CASS_OPTION_CONTACT_POINTS, contact_point);
// }


  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      println!("foo");
      let mut session=session;
      let session = &mut session;
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
  }
}
