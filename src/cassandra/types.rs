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

use cassandra::error::CassError;

use self::libc::c_char;


#[allow(dead_code)]
pub struct CassValue {
  pub cass_value:*const self::cass_internal_api::CassValue
}

#[allow(dead_code)]
pub struct CassString {
  pub cass_string:self::cass_internal_api::CassString
}

#[allow(dead_code)]
pub struct CassUuid {
  pub cass_uuid:self::cass_internal_api::CassUuid
}

#[allow(dead_code)]
pub struct CassInet {
  pub cass_inet:*mut self::cass_internal_api::CassInet
}

#[allow(dead_code)]
pub struct CassIterator {
  pub cass_iterator:*mut self::cass_internal_api::CassIterator
}

#[allow(dead_code)]
pub struct CassBytes {
  pub cass_bytes:*mut self::cass_internal_api::CassBytes
}

#[allow(dead_code)]
pub struct CassDecimal {
  pub cass_decimal:*mut self::cass_internal_api::CassDecimal
}

#[allow(dead_code)]
pub struct CassValueType {
  pub cass_value_type:self::cass_internal_api::CassValueType
}

#[allow(dead_code)]
pub struct CassBoolType {
  pub cass_bool_type:self::cass_internal_api::cass_bool_t
}

#[allow(dead_code)]
pub struct CassSizeType {
  pub cass_size_type:self::cass_internal_api::cass_size_t
}

#[allow(dead_code)]
pub struct CassDoubleType {
  pub cass_double_type:self::cass_internal_api::cass_double_t
}

pub struct CassFloatType {
  pub cass_float_type:self::cass_internal_api::cass_float_t
}

pub struct CassInt32Type {
  pub cass_int32_type:self::cass_internal_api::cass_int32_t
}

pub struct CassInt64Type {
  pub cass_int64_type:self::cass_internal_api::cass_int64_t
}

#[allow(dead_code)]
impl CassValue {
  pub fn string_init(null_terminated: CString) -> CassString {unsafe{
    CassString{cass_string:cass_internal_api::cass_string_init(null_terminated.as_ptr())}
  }}

  pub fn get_int32(self,mut output: cass_int32_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_int32(self.cass_value,&mut output)}
  }}

  pub fn get_int64(self, mut output: cass_int64_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_int64(self.cass_value,&mut output)}
  }}

  pub fn get_float(self, mut output: cass_float_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_float(self.cass_value,&mut output)}
  }}

  pub fn get_double(self, mut output: cass_double_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_double(self.cass_value,&mut output)}
  }}

  pub fn get_bool(self, mut output: cass_bool_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_bool(self.cass_value,&mut output)}
  }}

  pub fn get_uuid(self, output: CassUuid) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_uuid(self.cass_value,output.cass_uuid)}
  }}

  pub fn get_inet(self, output: CassInet) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_inet(self.cass_value,output.cass_inet)}
  }}

  pub fn get_string(self) -> Result<CassString,CassError> {unsafe{
    let my_str:*mut cass_internal_api::Struct_CassString_=&mut cass_internal_api::cass_string_init(self.cass_value as *const i8);
    let my_err = CassError{cass_error:cass_internal_api::cass_value_get_string(self.cass_value,my_str)};
    Ok(CassString{cass_string:*my_str})
  }}

  pub fn get_bytes(self, output: CassBytes) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_bytes(self.cass_value,output.cass_bytes)}
  }}

  pub fn get_decimal(self, output: CassDecimal) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_decimal(self.cass_value,output.cass_decimal)}
  }}

  pub fn cass_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_type(self.cass_value)}
  }}

  pub fn is_null(self) -> CassBoolType {unsafe{
    CassBoolType{cass_bool_type:cass_internal_api::cass_value_is_null(self.cass_value)}
  }}

  pub fn is_collection(self) -> CassBoolType {unsafe{
    CassBoolType{cass_bool_type:cass_internal_api::cass_value_is_collection(self.cass_value)}
  }}

  pub fn item_count(self) -> CassSizeType {unsafe{
    CassSizeType{cass_size_type:cass_internal_api::cass_value_item_count(self.cass_value)}
  }}

  pub fn primary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_primary_sub_type(self.cass_value)}
  }}

  pub fn secondary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_secondary_sub_type(self.cass_value)}
  }}

  pub fn cass_uuid_generate_time(output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_generate_time(output.cass_uuid);
  }}

  pub fn cass_uuid_from_time(time: cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_from_time(time,output.cass_uuid);
  }}

  pub fn cass_uuid_min_from_time(time: cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_min_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_max_from_time(time: cass_uint64_t, output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_max_from_time(time,output.cass_uuid)
  }}

  pub fn cass_uuid_generate_random(output: CassUuid) {unsafe{
    cass_internal_api::cass_uuid_generate_random(output.cass_uuid)
  }}

  pub fn cass_uuid_timestamp(uuid: CassUuid) -> cass_uint64_t {unsafe{
    cass_internal_api::cass_uuid_timestamp(uuid.cass_uuid)
  }}

  pub fn cass_uuid_version(uuid: CassUuid) -> cass_uint8_t {unsafe{
    cass_internal_api::cass_uuid_version(uuid.cass_uuid)
  }}

  pub fn cass_uuid_string(uuid: CassUuid, output: *mut c_char) {unsafe{
    cass_internal_api::cass_uuid_string(uuid.cass_uuid,output)
  }}

  pub fn cass_inet_init_v4(address: *const cass_uint8_t) -> CassInet {unsafe{
    CassInet{cass_inet:&mut cass_internal_api::cass_inet_init_v4(address)}
  }}

  pub fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet {unsafe{
    CassInet{cass_inet:&mut cass_internal_api::cass_inet_init_v6(address)}
  }}

  pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal {unsafe{
    CassDecimal{cass_decimal:&mut cass_internal_api::cass_decimal_init(scale,* varint.cass_bytes)}
  }}

  pub fn cass_bytes_init(data: *const cass_byte_t, size: cass_size_t) -> CassBytes {unsafe{
    CassBytes{cass_bytes:&mut cass_internal_api::cass_bytes_init(data,size)}
  }}

  pub fn string_init2(data: *const c_char, length: cass_size_t) -> CassString {unsafe{
    CassString{cass_string:cass_internal_api::cass_string_init2(data,length)}
  }}

  pub fn cass_iterator_from_collection(self) -> CassIterator {unsafe{
    CassIterator{cass_iterator:cass_internal_api::cass_iterator_from_collection(self.cass_value)}
  }}
}
