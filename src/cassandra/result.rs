extern crate cass_internal_api;

use self::cass_internal_api::cass_size_t;


use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;

use cassandra::types::CassString;
use cassandra::iterator::CassIterator;

mod cassandra {
  #[path="../types.rs"] pub mod types;
}

#[allow(dead_code)]
pub struct CassResult {
  pub cass_result:*const self::cass_internal_api::CassResult
}

impl Drop for CassResult {
  fn drop(&mut self) {unsafe{
    //println!("free my result");
    cass_internal_api::cass_result_free(self.cass_result)
  }}
}

impl Show for CassResult {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
     write!(f, "(Result:{})", self.cass_result)
    }
}

#[allow(dead_code)]
impl CassResult {
  pub fn row_count(&self) -> cass_size_t {unsafe{
    cass_internal_api::cass_result_row_count(self.cass_result)
  }}

  pub fn column_count(&self) -> cass_size_t {unsafe{
    cass_internal_api::cass_result_column_count(self.cass_result)
  }}

  pub fn column_name(&self, index: cass_size_t) -> CassString {unsafe{
    CassString{cass_string:cass_internal_api::cass_result_column_name(self.cass_result,index)}
  }}

  pub fn column_type(&self, index: cass_size_t) -> cass_internal_api::CassValueType {unsafe{
    cass_internal_api::cass_result_column_type(self.cass_result,index)
  }}

  pub fn first_row(&self) -> *const cass_internal_api::CassRow {unsafe{
    cass_internal_api::cass_result_first_row(self.cass_result)
  }}

  pub fn iterator(self) -> CassIterator {unsafe{
    CassIterator{cass_iterator:cass_internal_api::cass_iterator_from_result(self.cass_result)}
  }}
}
