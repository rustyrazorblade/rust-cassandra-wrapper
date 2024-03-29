extern crate cass_internal_api;

use cass_internal_api::cass_size_t;
use cass_internal_api::cass_bool_t;
use cass_internal_api::cass_int32_t;
use cass_internal_api::cass_int64_t;
use cass_internal_api::cass_float_t;
use cass_internal_api::cass_double_t;
use cass_internal_api::cass_byte_t;

use collection::CassCollection;
use error::CassError;
use types::CassDecimal;
use types::CassInet;
use types::CassUuid;
use types::CassBytes;
use types::CassString;
use result::CassResult;
use super::CassStatement;

use std::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;


impl Show for super::CassStatement {
   fn fmt(&self, f: &mut Formatter) -> Result {
     write!(f, "(Statement:{})", self)
    }
}

impl Drop for super::CassStatement {
  fn drop(&mut self) {unsafe{
    //println!("free my statement");
    cass_internal_api::cass_statement_free(self.cass_statement)
  }}
}

#[allow(dead_code)]
impl super::CassStatement {
pub fn new(statement_string: &CassString, parameter_count: cass_size_t) ->  CassStatement {unsafe{
  let statement = cass_internal_api::cass_statement_new(statement_string.cass_string,parameter_count);
  CassStatement{cass_statement:statement}
}}

pub fn build_from_string(statement_string:String, parameter_count: cass_size_t) -> CassStatement {unsafe{
  let query_cstring = statement_string.to_c_str();
  let query = cass_internal_api::cass_string_init(query_cstring.as_ptr());
  CassStatement{cass_statement:cass_internal_api::cass_statement_new(query,parameter_count)}
}}

#[test]
pub fn mytest () {
  println!{"test"};
}

pub fn set_paging_size( &mut self, page_size: ::libc::c_int) -> Option<CassError> {unsafe{
  //let ref mut myself = self;
  let error = CassError{cass_error:cass_internal_api::cass_statement_set_paging_size(self.cass_statement,page_size)};
  if error.is_error() {return Some(error)} else {return None}
}}

 pub fn set_paging_state(&mut self, result: &mut CassResult) -> Option<CassError> {unsafe{
   let error = CassError{cass_error:cass_internal_api::cass_statement_set_paging_state(self.cass_statement,result.cass_result)};
   if error.is_error() {return Some(error)} else {return None}
 }}

  pub fn bind_null(&mut self, index: cass_size_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_null(self.cass_statement,index)}
  }}

  pub fn bind_int32(&mut self, index: cass_size_t, value: cass_int32_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_int32(self.cass_statement,index,value)}
  }}

  pub fn bind_int64(&mut self, index: cass_size_t, value: cass_int64_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_int64(self.cass_statement,index,value)}
  }}

  pub fn bind_float(&mut self, index: cass_size_t, value: cass_float_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_float(self.cass_statement,index,value)}
  }}

  pub fn bind_double(&mut self, index: cass_size_t, value: cass_double_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_double(self.cass_statement,index,value)}
  }}

  pub fn bind_bool(&mut self, index: cass_size_t, value: cass_bool_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_bool(self.cass_statement,index,value)}
  }}

  pub fn bind_string(&mut self, index: cass_size_t, value: CassString) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_string(self.cass_statement,index,value.cass_string)}
  }}

  pub fn bind_bytes(&mut self, index: cass_size_t, value: CassBytes) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_bytes(self.cass_statement,index,value.cass_bytes)}
  }}

  pub fn bind_uuid(&mut self, index: cass_size_t, value: CassUuid) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_uuid(self.cass_statement,index,value.cass_uuid)}
  }}

  pub fn bind_inet(&mut self, index: cass_size_t, value: CassInet) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_inet(self.cass_statement,index,value.cass_inet)}
  }}

  pub fn bind_decimal(&mut self, index: cass_size_t, value: CassDecimal) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_decimal(self.cass_statement,index,value.cass_decimal)}
  }}

  pub fn bind_custom(&mut self, index: cass_size_t, size: cass_size_t, output: *mut *mut cass_byte_t) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_custom(self.cass_statement,index,size,output)}
  }}

  pub fn bind_collection(&mut self, index: cass_size_t, collection: CassCollection) -> CassError {unsafe{
    CassError{cass_error:cass_internal_api::cass_statement_bind_collection(self.cass_statement,index,&*collection.cass_collection)}
  }}
}

#[allow(dead_code)]
pub struct CassPrepared {
  pub cass_prepared:*const cass_internal_api::CassPrepared,
  //pub cass_statement:CassStatement
}

#[allow(dead_code)]
impl CassPrepared {
  // pub fn new(statement_string:CassString, parameter_count: cass_size_t, consistency: CassConsistency) -> CassStatement {unsafe{
  //   let prepared = CassPrepared{cass_prepared:None,cass_statement:* cass_internal_api::cass_statement_new(statement_string,parameter_count,consistency)};
  //   prepared.cass_statement
  // }}

  //  pub fn cass_prepared_free(prepared: *const CassPrepared);
  //   pub fn cass_prepared_bind(prepared: *const CassPrepared,
  //                             parameter_count: cass_size_t,
  //                             consistency: CassConsistency) ->
  //    *mut CassStatemstatement.cass_statement
   //
  // pub fn free(&self) {unsafe{
  //   cass_internal_api::cass_prepared_free(&(*self).cass_prepared);
  // }}


  pub fn bind(self, parameter_count: cass_size_t) -> CassStatement {unsafe{
    CassStatement{cass_statement:cass_internal_api::cass_prepared_bind(self.cass_prepared,parameter_count)}
  }}
}
