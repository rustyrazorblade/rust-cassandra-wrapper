extern crate cass_internal_api;

use cassandra::iterator::CassIterator;
use cassandra::types::CassValue;

#[path = "../cassandra/mod.rs"] mod cassandra
{
  #[path="../iterator.rs"] pub mod iterator;
}


pub struct CassRow {
  pub cass_row:self::cass_internal_api::CassRow
}

impl CassRow {
  pub fn cass_iterator_from_row(row: CassRow) -> CassIterator {unsafe{
    CassIterator{cass_iterator:cass_internal_api::cass_iterator_from_row(&row.cass_row)}
  }}

  pub fn get_column(self, index: u64) -> CassValue {unsafe{
    CassValue{cass_value:cass_internal_api::cass_row_get_column(&self.cass_row,index)}
  }}

}
