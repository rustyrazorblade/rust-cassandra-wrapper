extern crate cass_internal_api;
extern crate libc;

use libc::c_char;
use cass_internal_api::cass_size_t;

use future::CassFuture;
use super::CassSession;
use error::CassError;
use super::CassCluster;

#[allow(dead_code)]
impl super::CassCluster {

  fn new() -> CassCluster {unsafe{
    CassCluster{cass_cluster:cass_internal_api::cass_cluster_new()}
  }}

  pub fn create(contact_points:String) -> CassCluster {unsafe{
    let cluster = CassCluster::new();
    let err = cass_internal_api::cass_cluster_set_contact_points(cluster.cass_cluster, cass_internal_api::cass_string_init(contact_points.to_c_str().as_ptr()));
//    for contact_point in contact_points.iter() {
//      cluster.setopt(CASS_OPTION_CONTACT_POINTS, contact_point);
//    }
    cluster
  }}

//  pub fn setopt(&mut self, option: CassOption, data: &CString) -> cass_internal_api::CassError {unsafe{
//    cass_internal_api::cass_cluster_setopt( self.cass_cluster,option,data.as_ptr() as *const libc::c_void,data.len() as u64)
//  }}

//  pub fn getopt(&mut self, option: CassOption, data: *mut c_void, data_length: *mut cass_size_t) -> cass_internal_api::CassError {unsafe{
//    cass_internal_api::cass_cluster_getopt(&(*(self).cass_cluster),option,data,data_length)
//  }}

  pub fn connect_async(&mut self) -> CassFuture{unsafe{
    CassFuture{cass_future:cass_internal_api::cass_cluster_connect( self.cass_cluster)}
  }}

  pub fn connect(mut self) -> Result<CassSession,CassError> {
    let mut future: CassFuture = self.connect_async();
    future.wait();
    let rc = future.error_code();
    let session = future.get_session();

    if rc.is_error() {return Err(rc);} else {return Ok(session);}
  }

  pub fn connect_keyspace(&mut self, keyspace: *const c_char) -> CassFuture {unsafe{
    let fu = cass_internal_api::cass_cluster_connect_keyspace(self.cass_cluster,keyspace);
    CassFuture{cass_future:fu}
  }}

  pub fn free(&mut self) {unsafe{
    cass_internal_api::cass_cluster_free(self.cass_cluster)
  }}
}
