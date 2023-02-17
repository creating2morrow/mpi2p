#[macro_use] extern crate rocket;
use rocket::serde::json::{Json};
use rocket::response::status::Custom;
use rocket::http::Status;

use mpi2p::*;

#[cfg(test)] mod tests;

/*
 TODO:
   - create_product
   - get_product
   - update_product
   - create_order
   - get_order
   - update_order (multsig stuff is here (T_T))
*/

// JSON APIs

/// Get payment API version
#[get("/version")]
async fn get_version() -> Custom<Json<reqres::XmrApiVersionResponse>> {
    let res: reqres::XmrRpcVersionResponse = get_xmr_version().await;
    let version: i32 = res.result.version;
    Custom(Status::Accepted, Json(reqres::XmrApiVersionResponse { version }))
}

/// Return a single customer's information
#[get("/<address>")]
async fn get_customer(address: String) -> Custom<Json<reqres::GetCustomerResponse>> {
    let m_customer: models::Customer = find_customer(address).await;
    let res: reqres::GetCustomerResponse = reqres::GetCustomerResponse {
        id: m_customer.id, address: m_customer.c_xmr_address,
        name: m_customer.c_name, pgp: m_customer.c_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Get a single vendor's information
#[get("/<address>")]
async fn get_vendor(address: String) -> Custom<Json<reqres::GetVendorResponse>> {
    let m_vendor: models::Vendor = find_vendor(address).await;
    let res: reqres::GetVendorResponse = reqres::GetVendorResponse {
        id: m_vendor.id, active: m_vendor.active, address: m_vendor.v_xmr_address,
        description: m_vendor.v_description,
        name: m_vendor.v_name,pgp: m_vendor.v_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Login with wallet signature
#[get("/login/<corv>/<address>/<signature>")]
async fn login(address: String, corv: String, signature: String) -> Custom<Json<reqres::XmrApiVerifyResponse>> {
    let r_address: String = get_login_address(address, corv, signature).await;
    let badreq = "".to_string();
    if r_address == ApplicationErrors::LoginError.to_string() {
        Custom(Status::BadRequest, Json(reqres::XmrApiVerifyResponse { address: badreq }))
    } else {
        Custom(Status::Accepted, Json(reqres::XmrApiVerifyResponse { address: r_address }))
    }
}

/// Update customer information
#[patch("/update/<id>/<data>/<update_type>")]
async fn update_customer(id: i32, data: String, update_type: i32) -> Custom<Json<reqres::GetCustomerResponse>> {
    let m_customer: models::Customer = modify_customer(id, data, update_type).await;
    let res: reqres::GetCustomerResponse = reqres::GetCustomerResponse {
        id: m_customer.id, address: m_customer.c_xmr_address,
        name: m_customer.c_name, pgp: m_customer.c_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Update vendor information
#[patch("/update/<id>/<data>/<update_type>")]
async fn update_vendor(id: i32, data: String, update_type: i32) -> Custom<Json<reqres::GetVendorResponse>> {
    let m_vendor = modify_vendor(id, data, update_type).await;
    let res: reqres::GetVendorResponse = reqres::GetVendorResponse {
        id: m_vendor.id, active: m_vendor.active, address: m_vendor.v_xmr_address,
        description: m_vendor.v_description, name: m_vendor.v_name, pgp: m_vendor.v_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Create a product by passing vendor id
#[get("/create/<v_id>")]
async fn create_product(v_id: i32) -> Custom<Json<reqres::GetProductResponse>> {
    let m_product: models::Product = create_new_product(&v_id).await;
    let res: reqres::GetProductResponse = reqres::GetProductResponse {
        id: m_product.id, v_id: m_product.v_id, in_stock: m_product.in_stock,
        description: m_product.p_description, name: m_product.p_name,
        price: m_product.p_price, qty: m_product.qty,
    };
    Custom(Status::Accepted, Json(res))
}

// END JSON APIs

#[launch]
async fn rocket() -> _ {
    // pdgb and monero-wallet-rpc are required to be up at boot time
    establish_pgdb_connection().await;
    check_xmr_rpc_connection().await;
    if is_i2p_check_enabled() {
        check_i2p_connection().await;
    }
    log(LogLevel::INFO, "mpi2p is online").await;
    rocket::build()
        .mount("/", routes![login])
        .mount("/customer", routes![get_customer, update_customer])
        .mount("/vendor", routes![get_vendor, update_vendor])
        .mount("/product", routes![create_product, /*update_product*/])
        // .mount("/order", routes![get_order, update_order])
        .mount("/xmr", routes![get_version])
}
