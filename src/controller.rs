use rocket::{get, patch};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use crate::auth;
use crate::customer;
use crate::models;
use crate::monero;
use crate::order;
use crate::product;
use crate::reqres;
use crate::vendor;

// JSON APIs

/// Get payment API version
/// Protected: false
#[get("/version")]
pub async fn get_version() -> Custom<Json<reqres::XmrApiVersionResponse>> {
    let res: reqres::XmrRpcVersionResponse = monero::get_xmr_version().await;
    let version: i32 = res.result.version;
    Custom(
        Status::Accepted,
        Json(reqres::XmrApiVersionResponse { version }),
    )
}

/// Return a single customer's information
/// Protected: true
#[get("/<address>/<signature>")]
pub async fn get_customer(
    address: String,
    signature: String,
) -> Custom<Json<reqres::GetCustomerResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        let res: reqres::GetCustomerResponse = Default::default();
        return Custom(Status::Unauthorized, Json(res));
    }
    let m_customer: models::Customer = customer::find(address).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetCustomerResponse::build(m_customer)),
    )
}

/// Get a single vendor's information
/// Protected: true
#[get("/<address>/<signature>")]
pub async fn get_vendor(address: String, signature: String) -> Custom<Json<reqres::GetVendorResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    let m_vendor: models::Vendor = vendor::find_vendor(address).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetVendorResponse::build(m_vendor)),
    )
}

/// Login with wallet signature
#[get("/login/<corv>/<address>/<signature>")]
pub async fn login(
    address: String,
    corv: String,
    signature: String,
) -> Custom<Json<reqres::GetAuthResponse>> {
    let m_auth: models::Authorization = auth::get_login(address, corv, signature).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetAuthResponse::build(m_auth)),
    )
}

/// Update customer information
#[patch("/<address>/<signature>/update/<data>/<update_type>")]
pub async fn update_customer(
    address: String,
    data: String,
    signature: String,
    update_type: i32,
) -> Custom<Json<reqres::GetCustomerResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    let c: models::Customer = customer::find(address).await;
    let m_customer: models::Customer = customer::modify(c.cid, data, update_type).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetCustomerResponse::build(m_customer)),
    )
}

/// Update vendor information
#[patch("/<address>/<signature>/update/<data>/<update_type>")]
pub async fn update_vendor(
    address: String,
    signature: String,
    data: String,
    update_type: i32,
) -> Custom<Json<reqres::GetVendorResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    let v: models::Vendor = vendor::find_vendor(address).await;
    let m_vendor: models::Vendor = vendor::modify_vendor(v.vid, data, update_type).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetVendorResponse::build(m_vendor)),
    )
}

/// Create a product by passing vendor creds
#[get("/<address>/<signature>/create")]
pub async fn create_product(
    address: String,
    signature: String,
) -> Custom<Json<reqres::GetProductResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        let res: reqres::GetProductResponse = Default::default();
        return Custom(Status::Unauthorized, Json(res));
    }
    let v: models::Vendor = vendor::find_vendor(address).await;
    let m_product: models::Product = product::create(v.vid).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetProductResponse::build(m_product)),
    )
}

/// Get all products by passing vendor id
#[get("/<address>/<signature>")]
pub async fn get_vendor_products(
    address: String,
    signature: String,
) -> Custom<Json<reqres::GetVendorProductsResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    let m_vendor: models::Vendor = vendor::find_vendor(address).await;
    let m_products: Vec<models::Product> = product::find_all(m_vendor.vid).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetVendorProductsResponse::build(m_products)),
    )
}

/// Update product information
#[patch("/<address>/<signature>/update/<data>/<update_type>")]
pub async fn update_product(
    address: String,
    signature: String,
    data: String,
    update_type: i32,
) -> Custom<Json<reqres::GetProductResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    let v: models::Vendor = vendor::find_vendor(address).await;
    let m_product: models::Product = product::modify(v.vid, data, update_type).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetProductResponse::build(m_product)),
    )
}

/// Initialize order
#[get("/<address>/<signature>/create/<pid>")]
pub async fn initialize_order(
    address: String,
    signature: String,
    pid: String,
) -> Custom<Json<reqres::InitializeOrderResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    // get the cid from the address after verification
    let m_customer = customer::find(address).await;
    let temp_pid = String::from(&pid);
    let m_order: models::Order = order::create(m_customer.cid, temp_pid).await;
    Custom(
        Status::Accepted,
        Json(reqres::InitializeOrderResponse::build(pid, m_order)),
    )
}

// update order

// Get all orders by passing vendor id

// Get all orders by passing customer id

// END JSON APIs
