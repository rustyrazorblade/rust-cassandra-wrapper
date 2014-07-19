#![feature(globs)]
#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate libc;

use cassandra::cluster::*;
use cassandra::statement::*;
use cassandra::types::*;

use cassandra::types::CassString;

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

#[deriving(Show)]
pub struct Simple {
  pub keyspace_name: CassString,
  pub durable_writes: bool,
  pub strategy_class: CassString,
  pub strategy_options: CassString
}

fn main()  {
  let mut cluster = CassCluster::new();
  let contact_points = ["127.0.0.1".to_string().to_c_str()];

  for contact_point in contact_points.iter() {
    cluster.setopt(CASS_OPTION_CONTACT_POINTS, contact_point);
  }

  let mut session_future = cluster.connect_async();
  session_future.wait();


  if !session_future.error_code().is_error() {
    let session = session_future.get_session();

    let query_string = "SELECT * FROM system.schema_keyspaces;".to_string();
    let statement = CassStatement::build_from_string(query_string, 6, CASS_CONSISTENCY_ONE);

    let mut result_future = session.execute_async(statement);
    result_future.wait();

    if!result_future.error_code().is_error() {
      let result = result_future.get_result();
      let mut rows = result.iterator();

      let foo:String = "abc2".to_string();
      let mut output:Simple = Simple {
          keyspace_name:CassValue::string_init("abc".to_string()),
          durable_writes:false,
          strategy_class:CassValue::string_init("def".to_string()),
          strategy_options:CassValue::string_init("ghi".to_string()),
        };
        while rows.next() {
          let row = rows.get_row();
          match row.get_column(0).get_string() {Err(err) => println!("{}0-",err),Ok(col) => output.keyspace_name=col};
          match row.get_column(1).get_bool() {Err(err) => println!("{}1-",err),Ok(col) => output.durable_writes=col};
          match row.get_column(2).get_string() {Err(err) => println!("{}2-",err),Ok(col) => output.strategy_class=col};
          match row.get_column(3).get_string() {Err(err) => println!("{}3-",err),Ok(col) => output.strategy_options=col};

         println!("output:{}",output);
       }

    } else {
      //let message = result_future.error_message();
    }

    let mut close_future = session.close_async();
    close_future.wait();
  } else {
    let message = session_future.error_message();
    println!("Error: {}", message);
  }
}
