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
use cassandra::batch::CassBatch;

use cassandra::result::CassResult;

use std::c_str::CString;

use cassandra::types::CassString;

use collections::Deque;

use std::collections::DList;

use cassandra::cluster::CASS_OPTION_CONTACT_POINTS;
use cassandra::consistency::CASS_CONSISTENCY_ONE;


use std::kinds::marker::NoCopy;

#[path = "../cassandra/mod.rs"] mod cassandra {
  #[path="../statement.rs"] pub mod statement;
  #[path="../batch.rs"] pub mod batch;
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

struct Pair {
    key:String,
    value:String
}

fn prepare_insert_into_batch(session:CassSession) -> Result<CassPrepared,CassError> {
  let query = CassValue::string_init(&"INSERT INTO examples.pairs (key, value) VALUES (?, ?)".to_string());

  let mut future = session.prepare(query);
  future.wait();

   if(future.error_code().is_error()) {
     println!("error: {}",future.error_code());
     return Err(future.error_code());
   } else {
    let prepared = future.get_prepared();
    return Ok(prepared);
   }
}

fn insert_into_batch_with_prepared(session:CassSession , prepared:CassPrepared, pairs:DList<Pair>) -> CassError {
  let batch = CassBatch::new(cassandra::consistency::CASS_CONSISTENCY_ONE,cass_internal_api::CASS_BATCH_TYPE_LOGGED);
  for pair in pairs.iter() {
    let mut statement = prepared.bind(2);
    statement.bind_string(0, CassValue::string_init(&pair.key));
    statement.bind_string(1, CassValue::string_init(&pair.value));
    batch.add_statement(statement);
  }
  let st2 = CassStatement::new(CassValue::string_init(&"INSERT INTO examples.pairs (key, value) VALUES ('c', '3')".to_string()),0,1/*fixme consistency*/);
  batch.add_statement(st2);

  {
    let mut statement = CassStatement::new(CassValue::string_init(&"INSERT INTO examples.pairs (key, value) VALUES (?, ?)".to_string()),2,1/*fixme consistency*/);
    statement.bind_string(0, CassValue::string_init(&"d".to_string()));
    statement.bind_string(1, CassValue::string_init(&"4".to_string()));
    batch.add_statement(statement);
  }

  let mut future = session.execute_batch(batch);
  future.wait();
  if(!future.error_code().is_error()) {
  } else {
    let prepared = future.get_prepared();
  }

  return future.error_code();
}

fn main() {
let contact_points = vec!("127.0.0.1".to_string().to_c_str());

  let mut cluster = CassCluster::create(contact_points);

  let mut pairs:DList<Pair> = DList::new();
  pairs.push_front(Pair{key:"a".to_string(), value:"1".to_string()});
  pairs.push_front(Pair{key:"b".to_string(), value:"2".to_string()});

  let (rc,session) = cluster.connect();
  if rc.is_error() {/*fixme*/}
  let ks_statement = CassValue::string_init(&"CREATE KEYSPACE examples WITH replication = {'class': 'SimpleStrategy', 'replication_factor': '3' };".to_string());
  session.execute(CassStatement::new(ks_statement,0,1));

  let table_statement = CassValue::string_init(&"CREATE TABLE examples.pairs (key text, value text, PRIMARY KEY (key));".to_string());
  session.execute(CassStatement::new(table_statement,0,1));
  let response = prepare_insert_into_batch(session);
  match response {
    Err(fail) => println!("fail: {}",fail),
    Ok(result) => {insert_into_batch_with_prepared(session, result, pairs);}
  }
}
