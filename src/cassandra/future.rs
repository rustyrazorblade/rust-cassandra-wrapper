extern crate cass_internal_api;

use self::cass_internal_api::CassSession;
use self::cass_internal_api::CassPrepared;
use self::cass_internal_api::CassResult;
use self::cass_internal_api::CassString;
use self::cass_internal_api::CassError;
use self::cass_internal_api::cass_bool_t;
use self::cass_internal_api::cass_duration_t;
use self::cass_internal_api::Struct_CassFuture_;

#[allow(dead_code)]
pub struct CassFuture {
  pub cass_future:*mut Struct_CassFuture_,
}

#[allow(dead_code)]
impl CassFuture {
  pub fn free(&mut self) {unsafe{
    self::cass_internal_api::cass_future_free((self).cass_future)
  }}

  pub fn ready(&mut self) -> cass_bool_t {unsafe{
    cass_internal_api::cass_future_ready((self).cass_future)
  }}

  pub fn wait(&mut self) {unsafe{
    cass_internal_api::cass_future_wait((self).cass_future)
  }}

  pub fn timed(&mut self, timeout: cass_duration_t) -> cass_bool_t {unsafe{
    cass_internal_api::cass_future_wait_timed((self).cass_future,timeout)
  }}

  pub fn session(&mut self) -> *mut CassSession {unsafe{
    cass_internal_api::cass_future_get_session( (self).cass_future)
  }}

  pub fn result(&mut self) -> *const CassResult {unsafe{
    cass_internal_api::cass_future_get_result( (self).cass_future)
  }}

  pub fn get_prepared(&mut self) -> *const CassPrepared {unsafe{
    cass_internal_api::cass_future_get_prepared( (self).cass_future)
  }}

  pub fn error_code(&mut self) -> CassError {unsafe{
    cass_internal_api::cass_future_error_code( (self).cass_future)
  }}

  pub fn error_message(&mut self) -> CassString {unsafe{
    cass_internal_api::cass_future_error_message( (self).cass_future)
  }}
}
