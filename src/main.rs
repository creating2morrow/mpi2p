#[macro_use] extern crate rocket;

use diesel::prelude::*;
use mpi2p::*;

#[cfg(test)] mod tests;

/*
 TODO:
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
   - i2p installation and setup
*/

#[get("/version")]
async fn version() -> String {
    get_xmr_version().await
}

#[get("/login/<address>/<signature>")]
async fn customer_login(address: String, signature: String) -> String {
    use self::schema::customers::dsl::*;
    let check_sig: String = verify_signature(address, signature).await;
    let connection = &mut establish_pgdb_connection().await;
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(&check_sig))
        .load::<models::Customer>(connection);
    
    // TODO: add to db on initial login
    match results {
        Ok(r) => println!("Customer exists: {:?}", r),
        _=> println!("Customer does not exists")
    }
    
    check_sig.to_string()
}

#[launch]
async fn rocket() -> _ {
    // pdgb and monero-wallet-rpc are required to be up at boot time
    establish_pgdb_connection().await;
    check_xmr_rpc_connection().await;
    rocket::build()
        .mount("/customer", routes![customer_login])
        .mount("/xmr", routes![version])
}
