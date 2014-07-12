extern crate cass_internal_api;
extern crate libc;

use self::libc::c_char;

#[allow(dead_code)]
pub static CASS_OK:u32 = cass_internal_api::CASS_OK;

#[allow(dead_code)]
pub struct CassError {
  pub cass_error:cass_internal_api::CassError
}

#[allow(dead_code)]
impl CassError {
  pub fn new (err:u32) -> CassError {
    CassError{cass_error:err}
  }


  pub fn cass_error_desc(&self) -> *const c_char {unsafe{
    cass_internal_api::cass_error_desc(self.cass_error)
  }}
}
