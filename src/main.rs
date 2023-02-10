#[macro_use] extern crate rocket;

use mpi2p::*;

#[cfg(test)] mod tests;

/*
 TODO:
   - md5 digest auth module
   - get_customer
   - update_customer
   - create_product
   - get_product
   - update_product
   - get_vendor
   - update_vendor
   - create_order
   - get_order
   - update_order
   - i2p connection check
*/


// HTML

// TODO: build html template injector, or should it be another microservice?

// END HTML

// JSON APIs
#[get("/version")]
async fn get_version() -> String {
    get_xmr_version().await
}

// TODO: use enum to have a single login entry point
#[get("/login/<address>/<signature>")]
async fn login_customer(address: String, signature: String) -> String {
    verify_customer_login(address, signature).await
}

#[get("/login/<address>/<signature>")]
async fn login_vendor(address: String, signature: String) -> String {
    verify_vendor_login(address, signature).await
}
// END JSON APIs

#[launch]
async fn rocket() -> _ {
    // pdgb and monero-wallet-rpc are required to be up at boot time
    establish_pgdb_connection().await;
    check_xmr_rpc_connection().await;
    // TODO: check_i2p_connection().await;
    log(LogLevel::INFO, "mpi2p is online").await;
    rocket::build()
        .mount("/customer", routes![login_customer])
        .mount("/vendor", routes![login_vendor])
        .mount("/xmr", routes![get_version])
}
