// #![feature(globs)]
// extern crate cass_internal_api;
// extern crate cassandra;
//
// use cass_internal_api::*;
// use cassandra::*;
// use cassandra::Basic;
// use cassandra::connect_session;
// use cassandra::execute_query;
// use cassandra::insert_into_basic;
// use cassandra::select_from_basic;
//
 fn main()  {
   println!{"main"}
 }
//   unsafe {
//     let cluster:*mut CassCluster = cassandra::create_cluster();
//
//     let input:Basic = Basic{bln:cass_true > 0, dbl:0.001f64, flt:0.0002f32, i32:1, i64:2 };
//     let output:Basic=  Basic{bln:false, dbl:0.0f64,flt:0.0f32, i32:1, i64:2};
//
//     let (rc,session) = connect_session(cluster);
//     if rc != CASS_OK {
//       println!("rc={}",rc);
//       return
//     }
//     let rawString = "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";
//     execute_query(session, rawString.to_c_str());
//
//     let create_table_string = "CREATE TABLE examples.basic (key text, bln boolean, flt float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));";
//
//     execute_query(session, create_table_string.to_c_str());
//
//     insert_into_basic(session, "test".to_c_str(), input);
//     select_from_basic(session, "test".to_c_str(), output);
//
//   // assert!(input.bln == output.bln);
//   // assert!(input.flt == output.flt);
//   // assert!(input.dbl == output.dbl);
//   // assert!(input.i32 == output.i32);
//   // assert!(input.i64 == output.i64);
//   //
//     let close_future = cass_session_close(session);
//     cass_future_wait(close_future);
//     cass_cluster_free(cluster);
//   }
//
// }
