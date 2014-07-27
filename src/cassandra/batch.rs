extern crate cass_internal_api;
extern crate libc;
use CassStatement;
use CassError;
use super::CassBatch;

#[allow(non_camel_case_types)] pub type CassBatchType = libc::c_uint;
pub static CASS_BATCH_TYPE_LOGGED:u32 = cass_internal_api::CASS_BATCH_TYPE_LOGGED;


// pub impl ProtectedCass<T> {
//   fn get_protected_field(&self) -> *mut T {self._protected}
// }

impl super::CassBatch {
  pub fn get_protected_field(&self) -> *mut cass_internal_api::CassBatch {
    self._protected
  }
}


#[allow(dead_code)]
impl super::CassBatch {
  pub fn new(batch_type: CassBatchType) -> CassBatch {unsafe{
    CassBatch{_protected:cass_internal_api::cass_batch_new(batch_type)}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_batch_free(self._protected);
  }}

  pub fn add_statement(self, statement: CassStatement) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_batch_add_statement(self._protected,statement.cass_statement)}
  }}
}
