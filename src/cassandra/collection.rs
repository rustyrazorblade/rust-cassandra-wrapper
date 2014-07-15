extern crate cass_internal_api;

use cassandra::error::CassError;
use cassandra::types::CassDecimal;
use cassandra::types::CassInet;
use cassandra::types::CassUuid;
use cassandra::types::CassBytes;
use cassandra::types::CassString;
use cassandra::types::CassBoolType;
use cassandra::types::CassDoubleType;
use cassandra::types::CassFloatType;
use cassandra::types::CassInt64Type;
use cassandra::types::CassInt32Type;
use cassandra::types::CassSizeType;

use std::kinds::marker::NoCopy;

#[allow(dead_code)]
pub struct CassCollection {
  pub cass_collection:*mut cass_internal_api::CassCollection,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
impl CassCollection {
  pub fn new(item_count: CassSizeType) -> CassCollection {unsafe{
    CassCollection{cass_collection:cass_internal_api::cass_collection_new(item_count.cass_size_type),nocopy:NoCopy}
  }}

  pub fn free(collection: CassCollection) {unsafe{
    cass_internal_api::cass_collection_free(collection.cass_collection)
  }}

  pub fn append_int32(collection: CassCollection, value: CassInt32Type) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_int32(collection.cass_collection,value.cass_int32_type),nocopy:NoCopy}
  }}

  pub fn append_int64(collection: CassCollection, value: CassInt64Type) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_int64(collection.cass_collection,value.cass_int64_type),nocopy:NoCopy}
  }}

  pub fn append_float(collection: CassCollection, value: CassFloatType) -> CassError {unsafe{
      CassError{cass_error:cass_internal_api::cass_collection_append_float(collection.cass_collection,value.cass_float_type),nocopy:NoCopy}
  }}

  pub fn append_double(collection: CassCollection, value: CassDoubleType) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_double(collection.cass_collection,value.cass_double_type),nocopy:NoCopy}
  }}

  pub fn append_bool(collection: CassCollection, value: CassBoolType) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_bool(collection.cass_collection,value.cass_bool_type),nocopy:NoCopy}
  }}

  pub fn append_string(collection: CassCollection, value: CassString) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_string(collection.cass_collection,*value.cass_string),nocopy:NoCopy}
  }}

  pub fn append_bytes(collection: CassCollection, value: CassBytes) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_bytes(collection.cass_collection,value.cass_bytes),nocopy:NoCopy}
  }}

  pub fn append_uuid(collection: CassCollection, value: CassUuid) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_uuid(collection.cass_collection,value.cass_uuid),nocopy:NoCopy}
  }}

  pub fn append_inet(collection: CassCollection, value: CassInet) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_collection_append_inet(collection.cass_collection,value.cass_inet),nocopy:NoCopy}
    }}

  pub fn append_decimal(collection: CassCollection, value: CassDecimal) -> CassError {unsafe{
    CassError::new(cass_internal_api::cass_collection_append_decimal(collection.cass_collection,value.cass_decimal))
    }}
}
