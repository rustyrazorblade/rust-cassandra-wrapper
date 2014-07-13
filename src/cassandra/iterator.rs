#[allow(dead_code)]
extern crate cass_internal_api;

use cassandra::row::CassRow;
use cassandra::types::CassValue;

impl Drop for CassIterator {
  fn drop(&mut self) {unsafe{
    println!("free my iterator");
    cass_internal_api::cass_iterator_free(self.cass_iterator)
  }}
}

#[allow(dead_code)]
pub struct CassIterator {
  pub cass_iterator:*mut self::cass_internal_api::CassIterator
}

#[allow(dead_code)]
impl CassIterator {
  pub fn next(&mut self) -> bool {unsafe{
    cass_internal_api::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_row(&mut self) -> CassRow {unsafe{
    let row = cass_internal_api::cass_iterator_get_row(self.cass_iterator);
    CassRow{cass_row:*row}
  }}

  pub fn get_column(self) -> CassValue {unsafe{
    CassValue{cass_value:cass_internal_api::cass_iterator_get_column(self.cass_iterator)}
  }}

  pub fn get_value(self) -> CassValue {unsafe{
    CassValue{cass_value:cass_internal_api::cass_iterator_get_value(self.cass_iterator)}
  }}
}
