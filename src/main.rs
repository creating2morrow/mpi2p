#[macro_use] extern crate rocket;
use rocket::serde::json::{Json};
use rocket::response::status::Custom;
use rocket::http::Status;

use mpi2p::*;

#[cfg(test)] mod tests;

/*
 TODO:
   - error handling for json response
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
async fn get_version() -> Custom<Json<reqres::XmrApiVersionResponse>> {
    let res: reqres::XmrRpcVersionResponse = get_xmr_version().await;
    let version: i32 = res.result.version;
    Custom(Status { code: 200 }, Json(reqres::XmrApiVersionResponse { version }))
}

#[get("/login/<corv>/<address>/<signature>")]
async fn login(address: String, corv: String, signature: String) -> Custom<Json<reqres::XmrApiVerifyResponse>> {
    let r_address: String = get_login_address(address, corv, signature).await;
    let badreq = "".to_string();
    if r_address == ApplicationErrors::LoginError.to_string() {
        Custom(Status::BadRequest, Json(reqres::XmrApiVerifyResponse { address: badreq }))
    } else {
        Custom(Status { code: 200 }, Json(reqres::XmrApiVerifyResponse { address: r_address }))
    }
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
        .mount("/", routes![login])
        .mount("/xmr", routes![get_version])
}
