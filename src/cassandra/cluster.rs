extern crate cass_internal_api;
extern crate libc;

use std::c_str::CString;

use self::libc::c_void;
use self::libc::c_char;
use self::cass_internal_api::CassOption;
use self::cass_internal_api::cass_size_t;

use cassandra::session::CassSession;
use cassandra::future::CassFuture;
use cassandra::error::CassError;

use cassandra::future;

#[allow(dead_code)] pub static CASS_OPTION_CONTACT_POINTS:u32 = self::cass_internal_api::CASS_OPTION_CONTACT_POINTS;

#[allow(dead_code)]
pub struct CassCluster<'a> {
  cass_cluster:*mut cass_internal_api::CassCluster
}

#[allow(dead_code)]
impl<'a> CassCluster<'a> {

  pub fn new() -> CassCluster {unsafe{
    CassCluster{cass_cluster:cass_internal_api::cass_cluster_new()}
  }}

  pub fn setopt(&mut self, option: CassOption, data: &CString) -> cass_internal_api::CassError {unsafe{
    cass_internal_api::cass_cluster_setopt( self.cass_cluster,option,data.as_ptr() as *const libc::c_void,data.len() as u64)
  }}

  pub fn getopt(&mut self, option: CassOption, data: *mut c_void, data_length: *mut cass_size_t) -> cass_internal_api::CassError {unsafe{
    cass_internal_api::cass_cluster_getopt(&(*(self).cass_cluster),option,data,data_length)
  }}

  pub fn connect_async(&mut self) -> future::CassFuture{unsafe{
    future::CassFuture{cass_future:cass_internal_api::cass_cluster_connect( self.cass_cluster)}
  }}

  pub fn connect(mut self) -> (CassError,CassSession) {
    let mut future: CassFuture = self.connect_async();
    future.wait();
    let rc = future.error_code();
    let session = future.get_session();

    return (rc,session);
  }

  pub fn connect_keyspace(&mut self, keyspace: *const c_char) -> future::CassFuture {unsafe{
    let fu = cass_internal_api::cass_cluster_connect_keyspace(self.cass_cluster,keyspace);
    future::CassFuture{cass_future:fu}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_cluster_free(self.cass_cluster)
  }}
}
