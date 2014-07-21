// #![feature(globs)]
// #![feature(phase)]
// #[phase(plugin, link)] extern crate log;
// extern crate libc;
// extern crate collections;
// extern crate cass_internal_api;
//
// use cassandra::cluster::*;
// use cassandra::statement::*;
// use cassandra::types::*;
// use cassandra::session::CassSession;
// use cassandra::error::CassError;
// use std::collections::DList;
// use cassandra::future::CassFuture;
// use cassandra::batch::CassBatch;
//
// use cassandra::result::CassResult;
//
// use std::c_str::CString;
//
// use cassandra::types::CassString;
//
// use collections::Deque;
//
// use std::collections::DList;
//
// use cassandra::cluster::CASS_OPTION_CONTACT_POINTS;
// use cassandra::consistency::CASS_CONSISTENCY_ONE;
//
//
// use std::vec::Vec;
//
// use std::kinds::marker::NoCopy;
//
// #[path = "../cassandra/mod.rs"] mod cassandra {
//   #[path="../statement.rs"] pub mod statement;
//   #[path="../batch.rs"] pub mod batch;
//   #[path="../consistency.rs"] pub mod consistency;
//   #[path="../iterator.rs"] pub mod iterator;
//   #[path="../collection.rs"] pub mod collection;
//   #[path="../types.rs"] pub mod types;
//   #[path="../cluster.rs"] pub mod cluster;
//   #[path="../result.rs"] pub mod result;
//   #[path="../row.rs"] pub mod row;
//   #[path="../future.rs"] pub mod future;
//   #[path="../error.rs"] pub mod error;
//   #[path="../session.rs"] pub mod session;
// }
//
// static NUM_CONCURRENT_REQUESTS:uint = 100;
//
// fn insert_into_paging(session:CassSession, key:String) {
//   let query = CassValue::string_init("INSERT INTO paging (key, value) VALUES (?, ?);");
//
//
// let futures:Vec<CassFuture> = Vec::new();
// //  CassFuture* futures[NUM_CONCURRENT_REQUESTS];
//
//   //size_t i;
//   let i = 0;
//   while i < NUM_CONCURRENT_REQUESTS {
//   //    CassUuid key;
//      let value_buffer = [256];
//
//      let mut value_buffer:[u8, .. 256]=[0];
//     let statement = CassStatement::new(query, 2);
//
//     let key = CassValue::generate_timeuuid();
//     statement.bind_uuid(0, key);
//
//     //sprintf(value_buffer, "%u", (unsigned int)i);
//     statement.bind_string(1, CassValue::string_init(i.to_string()));
//
//     futures.push_back(session.execute(statement));
//     i+=1;
//   }
//
//   // for future in futures {
//   //   rc = cass_future_error_code(future);
//   //   if (rc != CASS_OK) {
//   //     print_error(future);
//   //   }
//   //
//   //   cass_future_free(future);
//   // }
// }
//
// fn select_from_paging(session:CassSession) {
//   let has_more_pages = true;
// //  const CassResult* result = NULL;
//   let query = CassValue::string_init("SELECT * FROM paging".to_string());
//   let statement = CassStatement::new(query, 0);
//
//   statement.set_paging_size(100);
//
//   while (has_more_pages) {
//     CassIterator* iterator;
//     CassFuture* future = cass_session_execute(session, statement);
//
//     if (cass_future_error_code(future) != 0) {
//       print_error(future);
//       break;
//     }
//
//     result = cass_future_get_result(future);
//     iterator = cass_iterator_from_result(result);
//
//     while (cass_iterator_next(iterator)) {
//     //  CassUuid key;
//     //  char key_buffer[CASS_UUID_STRING_LENGTH];
//     //  CassString value;
//     //  char value_buffer[256];
//
//       let row = iterator.get_row();
//       cass_value_get_uuid(row.get_column(0), key);
//       cass_uuid_string(key, key_buffer);
//
//       cass_value_get_string(cass_row_get_column(row, 1), &value);
//       memcpy(value_buffer, value.data, value.length);
//       value_buffer[value.length] = '\0';
//
//       printf("key: '%s' value: '%s'\n", key_buffer, value_buffer);
//     }
//
//     has_more_pages = cass_result_has_more_pages(result);
//
//     if (has_more_pages) {
//       cass_statement_set_paging_state(statement, result);
//     }
//
//     cass_iterator_free(iterator);
//     cass_result_free(result);
//
//   }
// }
//
// fn main() {
// //  CassError rc = 0;
// let contact_points = vec!("127.0.0.1".to_string().to_c_str());
//
//   let mut cluster = CassCluster::create(contact_points);
//
//   let session = cluster.connect();
//   if (rc != CASS_OK) {
//     return -1;
//   }
//
//   execute_query(session,
//                 "CREATE KEYSPACE examples WITH replication = { \
// 'class': 'SimpleStrategy', 'replication_factor': '3' };");
//
//
//   execute_query(session,
//                 "CREATE TABLE examples.paging (key timeuuid, \
// value text, \
// PRIMARY KEY (key));");
//
//   execute_query(session, "USE examples");
//
//   insert_into_paging(session, "test");
//   select_from_paging(session);
//
//   close_future = cass_session_close(session);
//   cass_future_wait(close_future);
//   cass_future_free(close_future);
//   cass_cluster_free(cluster);
//
//   return 0;
// }
