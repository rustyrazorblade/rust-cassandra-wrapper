extern crate cass_internal_api;
extern crate libc;

use std::c_str::CString;

use self::cass_internal_api::cass_size_t;
use self::cass_internal_api::cass_byte_t;
use self::cass_internal_api::cass_bool_t;
use self::cass_internal_api::cass_int32_t;
use self::cass_internal_api::cass_int64_t;
use self::cass_internal_api::cass_float_t;
use self::cass_internal_api::cass_double_t;
use self::cass_internal_api::cass_uint64_t;
use self::cass_internal_api::cass_uint8_t;

use self::cass_internal_api::CassValueType;
use self::cass_internal_api::CassError;
use self::cass_internal_api::CassString;
use self::cass_internal_api::CassInet;
use self::cass_internal_api::CassDecimal;
use self::cass_internal_api::CassBytes;
use self::cass_internal_api::CassUuid;
use self::cass_internal_api::CassIterator;

use self::libc::c_char;

#[allow(dead_code)]
pub fn string_init(null_terminated: CString) -> CassString {unsafe{
  cass_internal_api::cass_string_init(null_terminated .as_ptr() as *const i8)
}}

#[allow(dead_code)]
pub struct CassValue {
  cass_value:self::cass_internal_api::CassValue
}

#[allow(dead_code)]
impl CassValue {

  pub fn cass_value_get_int32(&self, output: *mut cass_int32_t) -> CassError {unsafe{
    cass_internal_api::cass_value_get_int32(&(*self).cass_value,output)
  }}

  pub fn cass_value_get_int64(& self, output: *mut cass_int64_t) -> CassError {unsafe{
    cass_internal_api::cass_value_get_int64(&(*self).cass_value,output)
  }}

  pub fn cass_value_get_float(& self, output: *mut cass_float_t) -> CassError {unsafe{
    cass_internal_api::cass_value_get_float(&(*self).cass_value,output)
  }}

  pub fn cass_value_get_double(& self, output: *mut cass_double_t) -> CassError {unsafe{
    cass_internal_api::cass_value_get_double(&(*self).cass_value,output)
  }}

  pub fn cass_value_get_bool(& self, output: *mut cass_bool_t) -> CassError {unsafe{
    cass_internal_api::cass_value_get_bool(&(*self).cass_value,output)
  }}

  pub fn cass_value_get_uuid(& self, output: CassUuid) -> CassError {unsafe{
    cass_internal_api::cass_value_get_uuid(&(*self).cass_value,output)
  }}

  pub fn get_inet(& self, output: *mut CassInet) -> CassError {unsafe{
    cass_internal_api::cass_value_get_inet(&(*self).cass_value,output)
  }}

  pub fn get_string(& self, output: *mut CassString) -> CassError {unsafe{
    cass_internal_api::cass_value_get_string(&(*self).cass_value,output)
  }}

  pub fn get_bytes(& self, output: *mut CassBytes) -> CassError {unsafe{
    cass_internal_api::cass_value_get_bytes(&(*self).cass_value,output)
  }}

  pub fn get_decimal(& self, output: *mut CassDecimal) -> CassError {unsafe{
    cass_internal_api::cass_value_get_decimal(&(*self).cass_value,output)
  }}

  pub fn cass_type(& self) -> CassValueType {unsafe{
    cass_internal_api::cass_value_type(&(*self).cass_value)
  }}

  pub fn is_null(& self) -> cass_bool_t {unsafe{
    cass_internal_api::cass_value_is_null(&(*self).cass_value)
  }}

  pub fn is_collection(& self) -> cass_bool_t {unsafe{
    cass_internal_api::cass_value_is_collection(&(*self).cass_value)
  }}

  pub fn item_count(& self) -> cass_size_t {unsafe{
    cass_internal_api::cass_value_item_count(&(*self).cass_value)
  }}

  pub fn primary_sub_type(& self) -> CassValueType {unsafe{
    cass_internal_api::cass_value_primary_sub_type(&(*self).cass_value)
  }}

  pub fn secondary_sub_type(& self) -> CassValueType {unsafe{
    cass_internal_api::cass_value_secondary_sub_type(&(*self).cass_value)
  }}

  pub fn cass_uuid_generate_time(output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_generate_time(output)
  }}

  pub fn cass_uuid_from_time(time: cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_from_time(time,output)
  }}

  pub fn cass_uuid_min_from_time(time: cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_min_from_time(time,output)
  }}

  pub fn cass_uuid_max_from_time(time: cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_max_from_time(time,output)
  }}

  pub fn cass_uuid_generate_random(output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_generate_random(output)
  }}

  pub fn cass_uuid_timestamp(uuid: CassUuid) -> cass_uint64_t {unsafe{
    cass_internal_api::cass_uuid_timestamp(uuid)
  }}

  pub fn cass_uuid_version(uuid: CassUuid) -> cass_uint8_t {unsafe{
    cass_internal_api::cass_uuid_version(uuid)
  }}

  pub fn cass_uuid_string(uuid: CassUuid, output: *mut c_char) {unsafe{
    cass_internal_api::cass_uuid_string(uuid,output)
  }}

  pub fn cass_inet_init_v4(address: *const cass_uint8_t) -> CassInet {unsafe{
    cass_internal_api::cass_inet_init_v4(address)
  }}

  pub fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet {unsafe{
    cass_internal_api::cass_inet_init_v6(address)
  }}

  pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal {unsafe{
    cass_internal_api::cass_decimal_init(scale,varint)
  }}

  pub fn cass_bytes_init(data: *const cass_byte_t, size: cass_size_t) -> CassBytes {unsafe{
    cass_internal_api::cass_bytes_init(data,size)
  }}

  pub fn string_init2(data: *const c_char, length: cass_size_t) -> CassString {unsafe{
    cass_internal_api::cass_string_init2(data,length)
  }}

  pub fn cass_iterator_from_collection(&self) -> *mut CassIterator {unsafe{
    cass_internal_api::cass_iterator_from_collection(&(*self).cass_value)
  }}
}
