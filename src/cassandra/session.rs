extern crate cass_internal_api;
use self::cass_internal_api::CassFuture;
use self::cass_internal_api::CassString;
use self::cass_internal_api::CassStatement;
use self::cass_internal_api::CassBatch;

#[allow(dead_code)]
pub struct CassSession {
  pub cass_session:cass_internal_api::CassSession
}

#[allow(dead_code)]
impl CassSession {
  pub fn cass_session_close(&mut self) -> *mut CassFuture {unsafe{
    cass_internal_api::cass_session_close(&mut(*self).cass_session)
  }}

  pub fn cass_session_prepare(&mut self, statement: CassString) -> *mut CassFuture {unsafe{
    cass_internal_api::cass_session_prepare(&mut(*self).cass_session,statement)
  }}

  pub fn cass_session_execute(&mut self, statement: *mut CassStatement) -> *mut CassFuture {unsafe{
    cass_internal_api::cass_session_execute(&mut(*self).cass_session,statement)
  }}

  pub fn cass_session_execute_batch(&mut self, batch: *mut CassBatch) -> *mut CassFuture {unsafe{
    cass_internal_api::cass_session_execute_batch(&mut(*self).cass_session,batch)
  }}
}
