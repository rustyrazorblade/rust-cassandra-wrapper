extern crate cass_internal_api;

pub fn cass_iterator_from_row(row: *const CassRow) -> *mut CassIterator {unsafe{
  cass_internal_api::cass_iterator_from_row(row)
}}

pub fn cass_row_get_column(row: *const CassRow, index: cass_size_t) -> *const CassValue {unsafe{
  cass_internal_api::cass_row_get_column(row,index)
}}
