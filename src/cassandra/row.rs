extern crate cass_internal_api;

use CassIterator;
use CassValue;

#[path = "../cassandra/mod.rs"] mod cassandra
{
  #[path="../iterator.rs"] pub mod iterator;
}


pub struct CassRow {
  pub cass_row:*const self::cass_internal_api::CassRow,
}

impl CassRow {
  #[allow(dead_code)]
  pub fn iterator(&mut self) -> CassIterator {unsafe{
    let ref cass_row = *self.cass_row;
    let my_iter = cass_internal_api::cass_iterator_from_row(cass_row);
    CassIterator{cass_iterator:my_iter}
  }}

  #[allow(dead_code)]
  pub fn get_column(&self, index: u64) -> CassValue {unsafe{
    let ref cass_row = *self.cass_row;
    CassValue{cass_value:cass_internal_api::cass_row_get_column(cass_row,index)}
  }}

}
