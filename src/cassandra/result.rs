extern crate cass_internal_api;

use self::cass_internal_api::cass_size_t;
use self::cass_internal_api::CassValueType;
use self::cass_internal_api::CassRow;
use self::cass_internal_api::CassString;
use self::cass_internal_api::CassIterator;

#[allow(dead_code)]
pub struct CassResult {
  pub cass_result:self::cass_internal_api::CassResult
}

#[allow(dead_code)]
impl CassResult {
  pub fn free(self) {unsafe{
    cass_internal_api::cass_result_free(&self.cass_result)
  }}

  pub fn row_count(&self) -> cass_size_t {unsafe{
    cass_internal_api::cass_result_row_count(&(*self).cass_result)
  }}

  pub fn column_count(&self) -> cass_size_t {unsafe{
    cass_internal_api::cass_result_column_count(&(*self).cass_result)
  }}

  pub fn column_name(&self, index: cass_size_t) -> CassString {unsafe{
    cass_internal_api::cass_result_column_name(&(*self).cass_result,index)
  }}

  pub fn column_type(&self, index: cass_size_t) -> CassValueType {unsafe{
    cass_internal_api::cass_result_column_type(&(*self).cass_result,index)
  }}

  pub fn first_row(&self) -> *const CassRow {unsafe{
    cass_internal_api::cass_result_first_row(&(*self).cass_result)
  }}

  pub fn iterator(&self) -> *mut CassIterator {unsafe{
    cass_internal_api::cass_iterator_from_result(&(*self).cass_result)
  }}
}
