// #![crate_name = "cassandra"]
// #![crate_type = "lib"]
extern crate libc;
extern crate cass_internal_api;



//use self::cluster::new;
mod cassandra {
#[path="../session.rs"] pub mod session;
#[path="../cluster.rs"] pub mod cluster;
#[path="../option.rs"] pub mod option;
#[path="../error.rs"] pub mod error;
#[path="../future.rs"] pub mod future;
#[path="../statement.rs"] pub mod statement;
#[path="../row.rs"] pub mod row;
#[path="../batch.rs"] pub mod batch;
#[path="../result.rs"] pub mod result;
#[path="../consistency.rs"] pub mod consistency;
#[path="../types.rs"] pub mod types;
#[path="../collection.rs"] pub mod collection;
#[path="../iterator.rs"] pub mod iterator;
}
