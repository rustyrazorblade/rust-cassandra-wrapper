extern crate cass_internal_api;

pub fn new(item_count: cass_size_t) -> *mut CassCollection {unsafe{
  cass_internal_api::cass_collection_new
}}

pub fn free(collection: *mut CassCollection) {
  cass_internal_api::cass_collection_free(collection)
}}

pub fn append_int32(collection: *mut CassCollection, value: cass_int32_t) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_int32(collection,value)
}}

pub fn append_int64(collection: *mut CassCollection, value: cass_int64_t) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_int64(collection,value)
}}

pub fn append_float(collection: *mut CassCollection, value: cass_float_t) -> CassError {unsafe{
    cass_internal_api::cass_collection_append_float(collection,value)
}}

pub fn append_double(collection: *mut CassCollection, value: cass_double_t) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_double(collection,value)
}}

pub fn append_bool(collection: *mut CassCollection, value: cass_bool_t) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_bool(collection,value)
}}

pub fn append_string(collection: *mut CassCollection, value: CassString) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_string(collection,value)
}}

pub fn append_bytes(collection: *mut CassCollection, value: CassBytes) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_bytes(collection,value)
}}

pub fn append_uuid(collection: *mut CassCollection, value: CassUuid) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_uuid(collection,value)
}}

pub fn append_inet(collection: *mut CassCollection, value: CassInet) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_inet(collection,value)
}}

pub fn append_decimal(collection: *mut CassCollection, value: CassDecimal) -> CassError {unsafe{
  cass_internal_api::cass_collection_append_decimal(collection,value)
}}
