extern crate cass_internal_api;
extern crate libc;


use self::libc::c_void;
use self::libc::c_char;
use self::cass_internal_api::CassOption;
use self::cass_internal_api::cass_size_t;
use self::cass_internal_api::CassError;

use self::cassandra::future;

mod cassandra {
  #[path="../future.rs"] pub mod future;
}


pub static CASS_OPTION_CONTACT_POINTS:u32 = self::cass_internal_api::CASS_OPTION_CONTACT_POINTS;


#[allow(dead_code)]
pub struct CassCluster<'a> {

  pub cass_cluster:*mut cass_internal_api::CassCluster
}

#[allow(dead_code)]
impl<'a> CassCluster<'a> {

  pub fn new() -> CassCluster {unsafe{
    CassCluster{cass_cluster:cass_internal_api::cass_cluster_new()}
  }}

     pub fn setopt(&mut self, option: CassOption, data: *const c_void, data_length: cass_size_t) -> CassError {unsafe{
        cass_internal_api::cass_cluster_setopt( (self.cass_cluster),option,data,data_length)
     }}

  pub fn getopt(&mut self, option: CassOption, data: *mut c_void, data_length: *mut cass_size_t) -> CassError {unsafe{
    cass_internal_api::cass_cluster_getopt(&(*(self).cass_cluster),option,data,data_length)
  }}

  pub fn connect(&mut self) -> future::CassFuture{unsafe{
    future::CassFuture{cass_future:cass_internal_api::cass_cluster_connect( (self).cass_cluster)}
  }}

  pub fn connect_keyspace(&mut self, keyspace: *const c_char) -> future::CassFuture {unsafe{
    let fu = cass_internal_api::cass_cluster_connect_keyspace(self.cass_cluster,keyspace);
    future::CassFuture{cass_future:fu}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_cluster_free(self.cass_cluster)
  }}
}
