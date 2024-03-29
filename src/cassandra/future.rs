extern crate cass_internal_api;

use cass_internal_api::cass_bool_t;
use cass_internal_api::cass_duration_t;

use super::CassSession;
use error::CassError;
use result::CassResult;
use statement::CassPrepared;
use types::CassString;

mod cassandra {
  #[path="../error.rs"] pub mod error;
}

#[allow(dead_code)]
pub struct CassFuture {
  pub cass_future:*mut  cass_internal_api::Struct_CassFuture_,
}


impl Drop for CassFuture {
  fn drop(&mut self) {unsafe{
      cass_internal_api::cass_future_free(self.cass_future)
  }
}}

#[allow(dead_code)]
impl CassFuture {


  pub fn ready(&self) -> cass_bool_t {unsafe{
    cass_internal_api::cass_future_ready(self.cass_future)
  }}

  pub fn wait(&mut self) {unsafe{
    cass_internal_api::cass_future_wait(self.cass_future)
  }}

  pub fn timed(&mut self, timeout: cass_duration_t) -> cass_bool_t {unsafe{
    cass_internal_api::cass_future_wait_timed(self.cass_future,timeout)
  }}

  pub fn get_session(&mut self) -> CassSession {unsafe{
    CassSession{cass_session:cass_internal_api::cass_future_get_session(self.cass_future)}
  }}

  pub fn get_result(&mut self) -> CassResult {unsafe{
    CassResult{cass_result:cass_internal_api::cass_future_get_result(self.cass_future)}
  }}

  pub fn get_prepared(&mut self) -> CassPrepared {unsafe{
    CassPrepared{cass_prepared:cass_internal_api::cass_future_get_prepared(self.cass_future)}
  }}

  pub fn error_code(&mut self) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_future_error_code(self.cass_future)}
  }}

  pub fn error_message(&mut self) -> CassString {unsafe{
    let msg = cass_internal_api::cass_future_error_message(self.cass_future);
    CassString{cass_string:msg}
  }}

  pub fn print_error(&mut self) {
    println!("Error: {}", "self");
  }
}
