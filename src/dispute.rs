// use crate::models::*;
// use crate::utils;
// use crate::schema;
// use diesel::prelude::*;
// use log::{debug, error, info};

// /// Create dispute
// pub async fn create(oid: String, signed_tx: String) -> Dispute {
//     info!("creating dispute");
//     use crate::schema::disputes;
//     let connection = &mut utils::establish_pgdb_connection().await;
//     let did: String = utils::generate_rnd();
//     let created: i64 = chrono::offset::Utc::now().timestamp();
//     let new_dispute = NewDispute {
//         did: &did,
//         created: &created,
//         orid: &oid,
//         tx_set: &&signed_tx,
//     };
//     debug!("insert dispute: {:?}", new_dispute);
//     diesel::insert_into(disputes::table)
//         .values(&new_dispute)
//         .get_result(connection)
//         .expect("error saving new dispute")
// }

// /// Dispute lookup for manual resolution
// pub async fn find(oid: String) -> Dispute {
//     use self::schema::disputes::dsl::*;
//     let connection = &mut utils::establish_pgdb_connection().await;
//     let result = disputes
//         .filter(schema::disputes::orid.eq(oid))
//         .first::<Dispute>(connection);
//     match result {
//         Ok(r) => r,
//         _ => {
//             error!("error finding auth");
//             Default::default()
//         }
//     }
// }
