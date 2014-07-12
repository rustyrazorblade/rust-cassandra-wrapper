extern crate cass_internal_api;

pub type CASS_CONSISTENCY = u32;

pub static CASS_CONSISTENCY_ONE:CASS_CONSISTENCY = cass_internal_api::CASS_CONSISTENCY_ONE;
pub static CASS_CONSISTENCY_TWO:CASS_CONSISTENCY = cass_internal_api::CASS_CONSISTENCY_TWO;

// #[allow(dead_code)]
// pub enum CassConsistency<u32> {
//   CASS_CONSISTENCY_ONE(u32)
// }
