#[macro_use] extern crate rocket;

pub mod reqres;
use mpi2p::*;

#[cfg(test)] mod tests;

/*
 TODO:
   - update order and vendor models / schemas to have v/o_xmr_address
   - md5 auth module
   - cmd line args
   - rpc connection check on startup
   - verify signature API for login
   - get_customer
   - update_customer
   - create_product
   - get_product
   - update_product
   - create_vendor
   - get_vendor
   - update_vendor
   - create_order
   - get_order
   - update_order
*/

#[get("/version")]
async fn xmr() -> String {
    get_xmr_version().await
}


#[launch]
async fn rocket() -> _ {
    establish_pgdb_connection().await;
    check_xmr_rpc_connection().await;
    rocket::build()
        .mount("/xmr", routes![xmr])
}
