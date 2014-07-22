#![feature(globs)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate libc;
extern crate collections;
extern crate cassandra;

use cassandra::CassStatement;
use cassandra::CassCluster;
use cassandra::CassResult;
use cassandra::CassFuture;
use cassandra::CassSession;
use cassandra::CassValue;

use std::vec::Vec;

static NUM_CONCURRENT_REQUESTS:uint = 100;

fn insert_into_paging(session:&mut CassSession, key:String) {
  let query = CassValue::string_init("INSERT INTO paging (key, value) VALUES (?, ?);".to_string());
  let mut futures:Vec<CassFuture> = Vec::new();
  for i in range(1,NUM_CONCURRENT_REQUESTS) {
    let mut statement = CassStatement::new(&query, 2);
    statement.bind_int32(0, i as i32 /*key*/);
    statement.bind_string(1, CassValue::string_init(i.to_string()));
    let future:CassFuture = session.execute_async(&mut statement);
    futures.push(future);
  }

  for future in futures.mut_iter() {
    future.wait();
  }
}

 fn select_from_paging(session:&mut CassSession) {
   let mut has_more_pages = true;
   let mut statement = CassStatement::build_from_string("SELECT * FROM paging".to_string(), 0);
   while has_more_pages {
     let mut future = session.execute_async(&mut statement);
     let mut result:CassResult = future.get_result();
     let mut iterator = result.iterator();
     let i:u32 = 1;
     while iterator.next() {
       let row = iterator.get_row();
       let key = row.get_column(0).get_int32();
       let value = row.get_column(1).get_string();
       println!("key: '{}' value: '{}'", key, value);

       if result.has_more_pages() {
         statement.set_paging_state(&mut result);
       } else {
         has_more_pages = false;
       }
     }
   }
 }

fn main() {
  let contact_points = "127.0.0.1".to_string();

  let cluster = CassCluster::create(contact_points);

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      let mut session=session;
      let result = session.execute(&mut CassStatement::build_from_string("CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };".to_string(),0));
      match result {
        Err(fail) => println!("fail: {}",fail),
        Ok(response) => {}
        }
      let result = session.execute(&mut CassStatement::build_from_string("CREATE TABLE examples.paging (key timeuuid, value text, PRIMARY KEY (key));".to_string(),0));

      let result=session.execute(&mut CassStatement::build_from_string("USE examples".to_string(),0));

      insert_into_paging(&mut session, "test".to_string());
      select_from_paging(&mut session);
      //    session.close();
    }
  }
}
