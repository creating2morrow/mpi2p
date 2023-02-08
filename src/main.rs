#[macro_use] extern crate rocket;

use mpi2p::*;

#[cfg(test)] mod tests;

/*
 TODO:
   - logger
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
   - i2p connection check
*/

#[get("/version")]
async fn get_version() -> String {
    get_xmr_version().await
}

#[get("/login/<address>/<signature>")]
async fn login_customer(address: String, signature: String) -> String {
    verify_customer_login(address, signature).await
}

#[launch]
async fn rocket() -> _ {
    // pdgb and monero-wallet-rpc are required to be up at boot time
    establish_pgdb_connection().await;
    check_xmr_rpc_connection().await;
    rocket::build()
        .mount("/customer", routes![login_customer])
        .mount("/xmr", routes![get_version])
}
