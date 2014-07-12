#[allow(dead_code)]
extern crate cass_internal_api;

use cassandra::row::CassRow;
use cassandra::types::CassValue;

#[allow(dead_code)]
pub struct CassIterator {
  pub cass_iterator:*mut self::cass_internal_api::CassIterator
}

#[allow(dead_code)]
impl CassIterator {
  pub fn free(self) {unsafe{
    cass_internal_api::cass_iterator_free(self.cass_iterator)
  }}

  pub fn next(self) -> bool {unsafe{
    cass_internal_api::cass_iterator_next(self.cass_iterator) > 0
  }}

  pub fn get_row(self) -> CassRow {unsafe{
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
