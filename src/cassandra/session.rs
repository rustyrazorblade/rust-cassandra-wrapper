extern crate cass_internal_api;

use cassandra::statement::CassStatement;
use cassandra::future::CassFuture;
use cassandra::error::CassError;
use cassandra::error::CASS_OK;
use cassandra::result::CassResult;

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
  pub fn close(self) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_close(self.cass_session)}
  }}

  pub fn build(self, statement: self::cass_internal_api::CassString) -> *mut self::cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_prepare(self.cass_session,statement)
  }}

  pub fn prepare(self, statement: self::cass_internal_api::CassString) -> *mut self::cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_prepare(self.cass_session,statement)
  }}

  pub fn execute(&self, statement:CassStatement) -> Result<CassResult,CassError> {

    let mut future:CassFuture = self.execute_async(statement);
    future.wait();
    let rc = future.error_code();
    if rc.cass_error != CASS_OK {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_async(&self, statement: CassStatement) -> CassFuture {unsafe{
    let future = cass_internal_api::cass_session_execute(self.cass_session,statement.cass_statement);
    CassFuture{cass_future:future}
  }}

  pub fn execute_batch(self, batch: *mut self::cass_internal_api::CassBatch) -> *mut self::cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_execute_batch(self.cass_session,batch)
  }}
}
