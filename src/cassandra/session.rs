extern crate cass_internal_api;

use cassandra::statement::CassStatement;
use cassandra::future::CassFuture;

#[allow(dead_code)]
pub struct CassSession {
  pub cass_session:*mut cass_internal_api::CassSession
}




mod cassandra {
#[path="../statement.rs"] pub mod statement;
#[path="../future.rs"] pub mod future;
}

#[allow(dead_code)]
impl CassSession {
  pub fn close(self) -> *mut self::cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_close(self.cass_session)
  }}

  pub fn prepare(self, statement: self::cass_internal_api::CassString) -> *mut self::cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_prepare(self.cass_session,statement)
  }}

  pub fn execute(self, statement: CassStatement) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_execute(self.cass_session,statement.cass_statement)}
  }}

  pub fn execute_batch(self, batch: *mut self::cass_internal_api::CassBatch) -> *mut self::cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_execute_batch(self.cass_session,batch)
  }}
}
