extern crate cass_internal_api;

use self::cass_internal_api::CassConsistency;
use self::cass_internal_api::CassBatchType;
use self::cass_internal_api::CassError;
use self::cass_internal_api::CassStatement;

#[allow(dead_code)]
pub struct CassBatch {
  cass_batch:self::cass_internal_api::CassBatch
}

#[allow(dead_code)]
impl CassBatch {
  pub fn new(consistency: CassConsistency, _type: CassBatchType) -> CassBatch {unsafe{
    CassBatch{cass_batch:*cass_internal_api::cass_batch_new(consistency,_type)}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_batch_free(&mut(*self).cass_batch)
  }}

  pub fn add_statement(&mut self, statement: *mut CassStatement) -> CassError {unsafe{
    cass_internal_api::cass_batch_add_statement(&mut(*self).cass_batch,statement)
  }}
}
