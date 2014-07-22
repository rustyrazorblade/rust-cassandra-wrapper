extern crate cass_internal_api;

use statement::CassStatement;
use future::CassFuture;
use error::CassError;
use result::CassResult;
use batch::CassBatch;

use CassString;

#[allow(dead_code)]
pub struct CassSession {
  pub cass_session:*mut cass_internal_api::CassSession
}

#[allow(dead_code)]
impl CassSession {
  pub fn close_async(&self) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_close(self.cass_session)}
  }}

  pub fn build(&self, statement: self::cass_internal_api::CassString) -> *mut self::cass_internal_api::CassFuture {unsafe{
    cass_internal_api::cass_session_prepare(self.cass_session,statement)
  }}

  pub fn prepare(&self, statement: CassString) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_prepare(self.cass_session,statement.cass_string)}
  }}

  pub fn execute(&self, statement:&CassStatement) -> Result<CassResult,CassError> {

    let mut future:CassFuture = self.execute_async(statement);
    future.wait();
    let rc = future.error_code();
    if rc.is_error() {
      return Err(rc);
    }
    return Ok(future.get_result());
  }

  pub fn execute_async(&self, statement: &CassStatement) -> CassFuture {unsafe{
    let future = cass_internal_api::cass_session_execute(self.cass_session,&*statement.cass_statement);
    CassFuture{cass_future:future}
  }}

  pub fn execute_batch(&self, batch: &CassBatch) -> CassFuture {unsafe{
    CassFuture{cass_future:cass_internal_api::cass_session_execute_batch(self.cass_session,&*batch.cass_batch)}
  }}
}
