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

use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;

use std::kinds::marker::NoCopy;

#[allow(dead_code)]
pub struct CassValue {
  pub cass_value:*const self::cass_internal_api::CassValue,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassString {
  pub cass_string:*mut self::cass_internal_api::CassString,
  pub nocopy:NoCopy
}

// impl Show for CassString {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//      write!(f, "{}", self.cass_string.to_string())
//     }
// }


#[allow(dead_code)]
pub struct CassUuid {
  pub cass_uuid:self::cass_internal_api::CassUuid,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassInet {
  pub cass_inet:self::cass_internal_api::CassInet,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassIterator {
  pub cass_iterator:*mut self::cass_internal_api::CassIterator,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassBytes {
  pub cass_bytes:self::cass_internal_api::CassBytes,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassDecimal {
  pub cass_decimal:self::cass_internal_api::CassDecimal,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassValueType {
  pub cass_value_type:self::cass_internal_api::CassValueType,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassBoolType {
  pub cass_bool_type:self::cass_internal_api::cass_bool_t,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassSizeType {
  pub cass_size_type:self::cass_internal_api::cass_size_t,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
pub struct CassDoubleType {
  pub cass_double_type:self::cass_internal_api::cass_double_t,
  pub nocopy:NoCopy
}

pub struct CassFloatType {
  pub cass_float_type:self::cass_internal_api::cass_float_t,
  pub nocopy:NoCopy
}

pub struct CassInt32Type {
  pub cass_int32_type:self::cass_internal_api::cass_int32_t,
  pub nocopy:NoCopy
}

pub struct CassInt64Type {
  pub cass_int64_type:self::cass_internal_api::cass_int64_t,
  pub nocopy:NoCopy
}

#[allow(dead_code)]
impl CassValue {
  pub fn string_init(null_terminated: CString) -> CassString {unsafe{
    let my_ptr=null_terminated.as_ptr();
    let ref mut inited = cass_internal_api::cass_string_init(my_ptr);
    CassString{cass_string:inited,nocopy:NoCopy}
  }}


  pub fn get_int32(self) ->  Result<cass_int32_t,CassError> {unsafe{
    let ref mut output:cass_int32_t=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_int32(self.cass_value,output),nocopy:NoCopy};
    if (err.cass_error == cass_internal_api::CASS_OK) {return Ok(*output)} else {return Err(err)}
  }}

pub fn get_int64(self) ->  Result<cass_int64_t,CassError> {unsafe{
  let ref mut output:cass_int64_t=0;
  let err = CassError{cass_error:cass_internal_api::cass_value_get_int64(self.cass_value,output),nocopy:NoCopy};
  if (err.cass_error == cass_internal_api::CASS_OK) {return Ok(*output)} else {return Err(err)}
}}

  pub fn get_float(self) ->  Result<cass_float_t,CassError> {unsafe{
    let ref mut output:cass_float_t=0.0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_float(self.cass_value,output),nocopy:NoCopy};
    if (err.cass_error == cass_internal_api::CASS_OK) {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_double(self) -> Result<f64,CassError> {unsafe{
    let ref mut output:f64=0.0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_double(self.cass_value,output),nocopy:NoCopy};
    if (err.cass_error == cass_internal_api::CASS_OK) {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_bool(self) -> Result<bool,CassError> {unsafe{
    let ref mut output:u32=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_bool(self.cass_value,output),nocopy:NoCopy};
    if (err.cass_error == cass_internal_api::CASS_OK) {return Ok(*output> 0)} else {return Err(err)}
  }}

  pub fn get_uuid(self, output: CassUuid) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_uuid(self.cass_value,output.cass_uuid),nocopy:NoCopy}
  }}

  pub fn get_inet(self, mut output: CassInet) -> CassError {unsafe{
    let ref mut cass_inet = output.cass_inet;
    CassError{cass_error:cass_internal_api::cass_value_get_inet(self.cass_value,cass_inet),nocopy:NoCopy}
  }}

  pub fn get_string(self) -> Result<CassString,CassError> {unsafe{
    let ref mut my_str:cass_internal_api::Struct_CassString_=cass_internal_api::cass_string_init(self.cass_value as *const i8);
    let my_err = CassError{cass_error:cass_internal_api::cass_value_get_string(self.cass_value,my_str),nocopy:NoCopy};
    Ok(CassString{cass_string:my_str,nocopy:NoCopy})
  }}

  pub fn get_bytes(self, mut output: CassBytes) -> CassError {unsafe{
    let ref mut my_bytes = output.cass_bytes;
    CassError{cass_error:cass_internal_api::cass_value_get_bytes(self.cass_value,my_bytes),nocopy:NoCopy}
  }}

  pub fn get_decimal(self, mut output: CassDecimal) -> CassError {unsafe{
    let ref mut my_decimal = output.cass_decimal;
    CassError{cass_error:cass_internal_api::cass_value_get_decimal(self.cass_value,my_decimal),nocopy:NoCopy}
  }}

  pub fn cass_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_type(self.cass_value),nocopy:NoCopy}
  }}

  pub fn is_null(self) -> CassBoolType {unsafe{
    CassBoolType{cass_bool_type:cass_internal_api::cass_value_is_null(self.cass_value),nocopy:NoCopy}
  }}

  pub fn is_collection(self) -> CassBoolType {unsafe{
    CassBoolType{cass_bool_type:cass_internal_api::cass_value_is_collection(self.cass_value),nocopy:NoCopy}
  }}

  pub fn item_count(self) -> CassSizeType {unsafe{
    CassSizeType{cass_size_type:cass_internal_api::cass_value_item_count(self.cass_value),nocopy:NoCopy}
  }}

  pub fn primary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_primary_sub_type(self.cass_value),nocopy:NoCopy}
  }}

  pub fn secondary_sub_type(self) -> CassValueType {unsafe{
    CassValueType{cass_value_type:cass_internal_api::cass_value_secondary_sub_type(self.cass_value),nocopy:NoCopy}
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
    CassInet{cass_inet:cass_internal_api::cass_inet_init_v4(address),nocopy:NoCopy}
  }}

  pub fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet {unsafe{
    CassInet{cass_inet:cass_internal_api::cass_inet_init_v6(address),nocopy:NoCopy}
  }}

  pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal {unsafe{
    CassDecimal{cass_decimal:cass_internal_api::cass_decimal_init(scale,varint.cass_bytes),nocopy:NoCopy}
  }}

  pub fn cass_bytes_init(data: *const cass_byte_t, size: cass_size_t) -> CassBytes {unsafe{
    CassBytes{cass_bytes:cass_internal_api::cass_bytes_init(data,size),nocopy:NoCopy}
  }}

  pub fn string_init2(data: *const c_char, length: cass_size_t) -> CassString {unsafe{
    let ref mut my_str = cass_internal_api::cass_string_init2(data,length);
    CassString{cass_string:my_str,nocopy:NoCopy}
  }}

  pub fn cass_iterator_from_collection(self) -> CassIterator {unsafe{
    CassIterator{cass_iterator:cass_internal_api::cass_iterator_from_collection(self.cass_value),nocopy:NoCopy}
  }}
}
