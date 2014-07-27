#![crate_name = "cassandra"]
#![crate_type = "lib"]
extern crate libc;
extern crate cass_internal_api;

pub use cluster::CassCluster;
pub use batch::CassBatch;
pub use error::CassError;
pub use future::CassFuture;
pub use statement::CassStatement;
pub use row::CassRow;
pub use result::CassResult;
pub use types::CassValue;
pub use types::CassString;
pub use collection::CassCollection;
pub use iterator::CassIterator;
pub use types::CassUuid;
pub use types::CassBytes;
pub use types::CassInet;
pub use types::CassInt32Type;
pub use types::CassDecimal;
pub use types::CassInt64Type;
pub use types::CassFloatType;
pub use types::CassDoubleType;
pub use types::CassBoolType;
pub use statement::CassPrepared;
pub use consistency::CASS_CONSISTENCY_ONE;
pub use batch::CASS_BATCH_TYPE_LOGGED;


#[allow(dead_code)]
pub struct CassSession {
  pub cass_session:*mut cass_internal_api::CassSession
}

mod session;
mod cluster;
mod option;
mod error;
mod future;
mod statement;
mod row;
mod batch;
mod result;
mod consistency;
mod types;
mod collection;
mod iterator;
