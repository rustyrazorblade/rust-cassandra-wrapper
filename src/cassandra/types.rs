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

use error::CassError;

use self::libc::c_char;

use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt;

#[allow(dead_code)]
pub struct CassValue {
  pub cass_value:*const cass_internal_api::CassValue
}

#[allow(dead_code)]
pub struct CassString {
  pub cass_string:cass_internal_api::CassString,
}

impl Show for CassString {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {unsafe{
     let cstr = CString::new(self.cass_string.data,false);
     write!(f, "{}", cstr.as_str().unwrap())
    }
}}

#[allow(dead_code)]
pub struct CassUuid {
  pub cass_uuid:cass_internal_api::CassUuid,
}

#[allow(dead_code)]
pub struct CassInet {
  pub cass_inet:cass_internal_api::CassInet,
}

#[allow(dead_code)]
pub struct CassIterator {
  pub cass_iterator:*mut cass_internal_api::CassIterator,
}

#[allow(dead_code)]
pub struct CassBytes {
  pub cass_bytes:cass_internal_api::CassBytes,
}

#[allow(dead_code)]
pub struct CassDecimal {
  pub cass_decimal:cass_internal_api::CassDecimal,
}

#[allow(dead_code)]
pub struct CassValueType {
  pub cass_value_type:cass_internal_api::CassValueType,
}

#[allow(dead_code)]
pub struct CassBoolType {
  pub cass_bool_type:cass_internal_api::cass_bool_t,
}

#[allow(dead_code)]
pub struct CassSizeType {
  pub cass_size_type:cass_internal_api::cass_size_t,
}

#[allow(dead_code)]
pub struct CassDoubleType {
  pub cass_double_type:cass_internal_api::cass_double_t,
}

#[allow(dead_code)]
pub struct CassFloatType {
  pub cass_float_type:cass_internal_api::cass_float_t,
}

#[allow(dead_code)]
pub struct CassInt32Type {
  pub cass_int32_type:cass_internal_api::cass_int32_t,
}

#[allow(dead_code)]
pub struct CassInt64Type {
  pub cass_int64_type:cass_internal_api::cass_int64_t,
}

#[allow(dead_code)]
impl CassValue {
// pub fn string_init(str: String) -> CassString {unsafe{
//   let query_cstring:CString = str.to_c_str();
//   let query:cass_internal_api::Struct_CassString_ = cass_internal_api::cass_string_init(query_cstring.as_ptr());
//   println!("initing:{}",query);
//   CassString{cass_string:query}
//   }}

// pub fn string_init(statement_string:String) -> CassString {unsafe{
//   let cstring = statement_string.clone().to_c_str();
//   let ref string = cass_internal_api::cass_string_init(cstring.as_ptr());
//   let wrapped = CassString{cass_string:*string,nocopy:NoCopy};
//   wrapped
// }}

pub fn string_init(string:String) -> CassString {unsafe{
  let cstr:CString = string.to_c_str();
  let unwrapped:*const i8 = cstr.unwrap();
  let string2 = cass_internal_api::cass_string_init(unwrapped);
  CassString{cass_string:string2}
}}


  pub fn get_string(self) ->  Result<CassString,CassError> {unsafe{
    let ref mut output:cass_internal_api::Struct_CassString_=cass_internal_api::cass_string_init(self.cass_value as *const i8);
    let err = CassError{cass_error:cass_internal_api::cass_value_get_string(self.cass_value,output)};
    let ref output = *output;
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(CassString{cass_string:*output})} else {return Err(err)}
  }}

  pub fn get_int32(self) ->  Result<cass_int32_t,CassError> {unsafe{
    let ref mut output:cass_int32_t=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_int32(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_int64(self) ->  Result<cass_int64_t,CassError> {unsafe{
    let ref mut output:cass_int64_t=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_int64(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
    }}

  pub fn get_float(self) ->  Result<cass_float_t,CassError> {unsafe{
    let ref mut output:cass_float_t=0.0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_float(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_double(self) -> Result<f64,CassError> {unsafe{
    let ref mut output:f64=0.0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_double(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output)} else {return Err(err)}
  }}

  pub fn get_bool(self) -> Result<bool,CassError> {unsafe{
    let ref mut output:u32=0;
    let err = CassError{cass_error:cass_internal_api::cass_value_get_bool(self.cass_value,output)};
    if err.cass_error == cass_internal_api::CASS_OK {return Ok(*output> 0)} else {return Err(err)}
  }}

  pub fn get_uuid(self, output: CassUuid) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_value_get_uuid(self.cass_value,output.cass_uuid)}
  }}

  pub fn get_inet(self, mut output: CassInet) -> CassError {unsafe{
    let ref mut cass_inet = output.cass_inet;
    CassError{cass_error:cass_internal_api::cass_value_get_inet(self.cass_value,cass_inet)}
  }}

  pub fn get_bytes(self, mut output: CassBytes) -> CassError {unsafe{
    let ref mut my_bytes = output.cass_bytes;
    CassError{cass_error:cass_internal_api::cass_value_get_bytes(self.cass_value,my_bytes)}
  }}

  pub fn get_decimal(self, mut output: CassDecimal) -> CassError {unsafe{
    let ref mut my_decimal = output.cass_decimal;
    CassError{cass_error:cass_internal_api::cass_value_get_decimal(self.cass_value,my_decimal)}
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

  //FIXME segfaults
  pub fn generate_timeuuid() -> CassUuid {unsafe{
    let output:cass_internal_api::CassUuid = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    cass_internal_api::cass_uuid_generate_time(output);
    CassUuid{cass_uuid:output}
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
    CassInet{cass_inet:cass_internal_api::cass_inet_init_v4(address)}
  }}

  pub fn cass_inet_init_v6(address: *const cass_uint8_t) -> CassInet {unsafe{
    CassInet{cass_inet:cass_internal_api::cass_inet_init_v6(address)}
  }}

  pub fn cass_decimal_init(scale: cass_int32_t, varint: CassBytes) -> CassDecimal {unsafe{
    CassDecimal{cass_decimal:cass_internal_api::cass_decimal_init(scale,varint.cass_bytes)}
  }}

  pub fn cass_bytes_init(data: *const cass_byte_t, size: cass_size_t) -> CassBytes {unsafe{
    CassBytes{cass_bytes:cass_internal_api::cass_bytes_init(data,size)}
  }}

  pub fn string_init2(data: *const c_char, length: cass_size_t) -> CassString {unsafe{
    let my_str = cass_internal_api::cass_string_init2(data,length);
    CassString{cass_string:my_str}
  }}

  pub fn cass_iterator_from_collection(self) -> CassIterator {unsafe{
    CassIterator{cass_iterator:cass_internal_api::cass_iterator_from_collection(self.cass_value)}
  }}
}
