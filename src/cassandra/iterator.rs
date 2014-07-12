#[allow(dead_code)]
extern crate cass_internal_api;

use self::cass_internal_api::cass_bool_t;


use self::cass_internal_api::CassRow;
use self::cass_internal_api::CassValue;

#[allow(dead_code)]
pub struct CassIterator {
  cass_iterator:self::cass_internal_api::CassIterator
}

#[allow(dead_code)]
impl CassIterator {
  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_iterator_free(&mut(*self).cass_iterator)
  }}

  pub fn next(&mut self) -> cass_bool_t {unsafe{
    cass_internal_api::cass_iterator_next(&mut(*self).cass_iterator)
  }}

  pub fn get_row(&mut self) -> *const CassRow {unsafe{
    cass_internal_api::cass_iterator_get_row(&mut(*self).cass_iterator)
  }}

  pub fn get_column(&mut self) -> *const CassValue {unsafe{
    cass_internal_api::cass_iterator_get_column(&mut(*self).cass_iterator)
  }}

  pub fn get_value(&mut self) -> *const CassValue {unsafe{
    cass_internal_api::cass_iterator_get_value(&mut(*self).cass_iterator)
  }}
}
