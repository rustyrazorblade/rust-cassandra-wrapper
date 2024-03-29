extern crate cass_internal_api;
extern crate libc;
use CassStatement;
use CassError;

#[allow(non_camel_case_types)] pub type CassBatchType = libc::c_uint;
pub static CASS_BATCH_TYPE_LOGGED:u32 = cass_internal_api::CASS_BATCH_TYPE_LOGGED;


#[allow(dead_code)]
pub struct CassBatch {
  pub cass_batch:*mut cass_internal_api::CassBatch,
}

#[allow(dead_code)]
impl CassBatch {
  pub fn new(batch_type: CassBatchType) -> CassBatch {unsafe{
    CassBatch{cass_batch:cass_internal_api::cass_batch_new(batch_type)}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_batch_free(self.cass_batch);
  }}

  pub fn add_statement(self, statement: CassStatement) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_batch_add_statement(self.cass_batch,statement.cass_statement)}
  }}
}
