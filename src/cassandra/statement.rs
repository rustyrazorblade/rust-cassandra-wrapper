extern crate cass_internal_api;
use self::cass_internal_api::CassString;
use self::cass_internal_api::CassConsistency;

use self::cass_internal_api::CassError;
use self::cass_internal_api::CassBytes;
use self::cass_internal_api::CassUuid;
use self::cass_internal_api::CassInet;
use self::cass_internal_api::CassDecimal;
use self::cass_internal_api::CassCollection;

use self::cass_internal_api::cass_size_t;
use self::cass_internal_api::cass_bool_t;
use self::cass_internal_api::cass_int32_t;
use self::cass_internal_api::cass_int64_t;
use self::cass_internal_api::cass_float_t;
use self::cass_internal_api::cass_double_t;
use self::cass_internal_api::cass_byte_t;

#[allow(dead_code)]
pub struct CassStatement {
  pub cass_statement:*mut cass_internal_api::CassStatement
}

#[allow(dead_code)]
impl CassStatement {
  pub fn new(statement_string:CassString, parameter_count: cass_size_t, consistency: CassConsistency) -> CassStatement {unsafe{
    let statement = CassStatement{cass_statement:cass_internal_api::cass_statement_new(statement_string,parameter_count,consistency)};
    statement
  }}

  pub fn free(self) {unsafe{
    cass_internal_api::cass_statement_free(self.cass_statement)
    }}

  pub fn bind_null(self, index: cass_size_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_null(self.cass_statement,index)
  }}

  pub fn bind_int32(self, index: cass_size_t, value: cass_int32_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_int32(self.cass_statement,index,value)
  }}

  pub fn bind_int64(self, index: cass_size_t, value: cass_int64_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_int64(self.cass_statement,index,value)
  }}

  pub fn bind_float(&mut self, index: cass_size_t, value: cass_float_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_float(self.cass_statement,index,value)
  }}

  pub fn bind_double(&mut self, index: cass_size_t, value: cass_double_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_double(self.cass_statement,index,value)
  }}

  pub fn bind_bool(&mut self, index: cass_size_t, value: cass_bool_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_bool(self.cass_statement,index,value)
  }}

  pub fn bind_string(&mut self, index: cass_size_t, value: CassString) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_string(self.cass_statement,index,value)
  }}

  pub fn bind_bytes(&mut self, index: cass_size_t, value: CassBytes) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_bytes(self.cass_statement,index,value)
  }}

  pub fn bind_uuid(&mut self, index: cass_size_t, value: CassUuid) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_uuid(self.cass_statement,index,value)
  }}

  pub fn bind_inet(&mut self, index: cass_size_t, value: CassInet) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_inet(self.cass_statement,index,value)
  }}

  pub fn bind_decimal(&mut self, index: cass_size_t, value: CassDecimal) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_decimal(self.cass_statement,index,value)
  }}

  pub fn bind_custom(&mut self, index: cass_size_t, size: cass_size_t, output: *mut *mut cass_byte_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_custom(self.cass_statement,index,size,output)
  }}

  pub fn bind_collection(&mut self, index: cass_size_t, collection: *const CassCollection, is_map: cass_bool_t) -> CassError {unsafe{
    cass_internal_api::cass_statement_bind_collection(self.cass_statement,index,collection, is_map)
  }}
}

#[allow(dead_code)]
pub struct CassPrepared {
  pub cass_prepared:cass_internal_api::CassPrepared,
  pub cass_statement:CassStatement
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


  // pub fn bind(&self, parameter_count: cass_size_t, consistency: CassConsistency) -> *mut CassStatement {unsafe{
  //   self.cass_statement.cass_statement = *cass_internal_api::cass_prepared_bind(&(*self).cass_prepared,parameter_count,consistency)
  // }}
}
