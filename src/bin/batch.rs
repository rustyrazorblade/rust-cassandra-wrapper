extern crate log;
extern crate libc;
extern crate collections;
extern crate cassandra;

use cassandra::CassStatement;
use cassandra::CassCluster;
use cassandra::CassValue;
use cassandra::CassSession;
use cassandra::CassPrepared;
use cassandra::CassError;
use cassandra::CassBatch;
use cassandra::CASS_BATCH_TYPE_LOGGED;

use collections::Deque;

use std::collections::DList;

struct Pair {
    key:String,
    value:String
}

fn prepare_insert_into_batch(session:CassSession) -> Result<CassPrepared,CassError> {
  let query = CassValue::string_init("INSERT INTO examples.pairs (key, value) VALUES (?, ?)".to_string());

  let mut future = session.prepare(query);
  future.wait();

   if future.error_code().is_error() {
     println!("error: {}",future.error_code());
     return Err(future.error_code());
   } else {
    let prepared = future.get_prepared();
    return Ok(prepared);
   }
}

fn insert_into_batch_with_prepared(session:CassSession , prepared:CassPrepared, pairs:&mut DList<Pair>) -> CassError {
  let batch = &mut CassBatch::new(CASS_BATCH_TYPE_LOGGED);
  for pair in pairs.mut_iter() {
    let mut statement = prepared.bind(2);
    let key = CassValue::string_init(pair.key.clone());
    let value = CassValue::string_init(pair.value.clone());
    statement.bind_string(0, key);
    statement.bind_string(1, value);
    batch.add_statement(statement);
  }
  let st2 = CassStatement::build_from_string("INSERT INTO examples.pairs (key, value) VALUES ('c', '3')".to_string(),0);
  batch.add_statement(st2);

  {
    let mut statement = CassStatement::build_from_string("INSERT INTO examples.pairs (key, value) VALUES (?, ?)".to_string(),2);
    statement.bind_string(0, CassValue::string_init("d".to_string()));
    statement.bind_string(1, CassValue::string_init("4".to_string()));
    batch.add_statement(statement);
  }

  let mut future = session.execute_batch(batch);
  future.wait();
  if !future.error_code().is_error() {
  } else {
    let prepared = future.get_prepared();
  }

  return future.error_code();
}

fn main() {
  let contact_points = "127.0.0.1".to_string();
  let cluster = CassCluster::create(contact_points);
  let pairs:&mut DList<Pair> = &mut DList::new();
  pairs.push_front(Pair{key:"a".to_string(), value:"1".to_string()});
  pairs.push_front(Pair{key:"b".to_string(), value:"2".to_string()});

  match cluster.connect() {
    Err(fail) => println!("fail: {}",fail),
    Ok(session) => {
      session.execute(&mut CassStatement::build_from_string("CREATE KEYSPACE examples WITH replication = {'class': 'SimpleStrategy', 'replication_factor': '3' };".to_string(),0));
      session.execute(&mut CassStatement::build_from_string("CREATE TABLE examples.pairs (key text, value text, PRIMARY KEY (key));".to_string(),0));
      let response = prepare_insert_into_batch(session);
      match response {
        Err(fail) => println!("fail: {}",fail),
        Ok(result) => {insert_into_batch_with_prepared(session, result, pairs);}
      }
    }
  }
}
