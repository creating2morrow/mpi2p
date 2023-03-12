use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get};

use crate::customer;
use crate::models;
use crate::monero;
use crate::reqres;
use crate::{auth};

// JSON APIs

/// Get payment API version
/// Protected: false
#[get("/version")]
pub async fn get_version() -> Custom<Json<reqres::XmrRpcVersionResponse>> {
    Custom(Status::Ok, Json(monero::get_version().await))
}

/// Return a single customer's information
/// Protected: true
#[get("/<_address>/<cvid>")]
pub async fn get_customer(
    _address: String,
    cvid: String,
    _token: auth::BearerToken,
) -> Custom<Json<reqres::GetCustomerResponse>> {
    let m_customer: models::Customer = customer::find(&cvid);
    Custom(
        Status::Ok,
        Json(reqres::GetCustomerResponse::build(m_customer)),
    )
}

// /// Get a single vendor's information
// /// Protected: true
// #[get("/<address>")]
// pub async fn get_vendor(
//     address: String,
//     _token: auth::BearerToken,
// ) -> Custom<Json<reqres::GetVendorResponse>> {
//     let m_vendor: models::Vendor = vendor::find(address).await;
//     Custom(Status::Ok, Json(reqres::GetVendorResponse::build(m_vendor)))
// }

/// Login with wallet signature
#[get("/login/<corv>/<address>/<signature>/<aid>/<cvid>")]
pub async fn login(
    address: String,
    aid: String,
    corv: String,
    cvid: String,
    signature: String,
) -> Custom<Json<reqres::GetAuthResponse>> {
    let m_auth: models::Authorization = 
        auth::get_login(address, aid, corv, cvid, signature).await;
    Custom(
        Status::Created,
        Json(reqres::GetAuthResponse::build(m_auth)),
    )
}

// /// Update customer information
// #[patch("/<address>/update/<data>/<update_type>")]
// pub async fn update_customer(
//     address: String,
//     data: String,
//     _token: auth::BearerToken,
//     update_type: i32,
// ) -> Custom<Json<reqres::GetCustomerResponse>> {
//     let c: models::Customer = customer::find(address).await;
//     let m_customer: models::Customer = customer::modify(c.cid, data, update_type).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetCustomerResponse::build(m_customer)),
//     )
// }

// /// Update vendor information
// #[patch("/<address>/update/<data>/<update_type>")]
// pub async fn update_vendor(
//     address: String,
//     _token: auth::BearerToken,
//     data: String,
//     update_type: i32,
// ) -> Custom<Json<reqres::GetVendorResponse>> {
//     let v: models::Vendor = vendor::find(address).await;
//     let m_vendor: models::Vendor = vendor::modify(v.vid, data, update_type).await;
//     Custom(Status::Ok, Json(reqres::GetVendorResponse::build(m_vendor)))
// }

// /// Create a product by passing vendor address
// #[get("/<address>/create")]
// pub async fn create_product(
//     address: String,
//     _token: auth::BearerToken,
// ) -> Custom<Json<reqres::GetProductResponse>> {
//     let v: models::Vendor = vendor::find(address).await;
//     let m_product: models::Product = product::create(v.vid).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetProductResponse::build(m_product)),
//     )
// }

// /// Get all products by passing vendor address
// #[get("/<address>")]
// pub async fn get_vendor_products(
//     address: String,
//     _token: auth::BearerToken,
// ) -> Custom<Json<reqres::GetVendorProductsResponse>> {
//     let m_vendor: models::Vendor = vendor::find(address).await;
//     let m_products: Vec<models::Product> = product::find_all(m_vendor.vid).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetVendorProductsResponse::build(m_products)),
//     )
// }

// /// Update product information
// #[patch("/<_address>/update/<pid>/<data>/<update_type>")]
// pub async fn update_product(
//     _address: String,
//     pid: String,
//     _token: auth::BearerToken,
//     data: String,
//     update_type: i32,
// ) -> Custom<Json<reqres::GetProductResponse>> {
//     let m_product: models::Product = product::modify(pid, data, update_type).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetProductResponse::build(m_product)),
//     )
// }

// /// Initialize order
// #[get("/<address>/create/<pid>")]
// pub async fn initialize_order(
//     address: String,
//     _token: auth::BearerToken,
//     pid: String,
// ) -> Custom<Json<reqres::GetOrderResponse>> {
//     // get the cid from the address after verification
//     let m_customer = customer::find(address).await;
//     let temp_pid = String::from(&pid);
//     let m_order: models::Order = order::create(m_customer.cid, temp_pid).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetOrderResponse::build(pid, m_order)),
//     )
// }

// /// Update order information
// #[patch("/<_address>/update/<pid>/<oid>/<data>/<update_type>")]
// pub async fn update_order(
//     _address: String,
//     oid: String,
//     pid: String,
//     _token: auth::BearerToken,
//     data: String,
//     update_type: i32,
// ) -> Custom<Json<reqres::GetOrderResponse>> {
//     let temp_pid: String = String::from(&pid);
//     let m_order: models::Order = order::modify(oid, pid, data, update_type).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetOrderResponse::build(temp_pid, m_order)),
//     )
// }

// /// Get all orders
// ///  by passing auth
// #[get("/<address>/<corv>")]
// pub async fn get_orders(
//     address: String,
//     corv: String,
//     _token: auth::BearerToken,
// ) -> Custom<Json<reqres::GetOrdersResponse>> {
//     let m_orders: Vec<models::Order> = order::find_all(address, corv).await;
//     Custom(Status::Ok, Json(reqres::GetOrdersResponse::build(m_orders)))
// }

// /// Create a dispute
// #[get("/<_address>/create/<oid>/<txset>")]
// pub async fn create_dispute(
//     _address: String,
//     _token: auth::BearerToken,
//     oid: String,
//     txset: String,
// ) -> Custom<Json<reqres::GetDisputeResponse>> {
//     let m_dispute: models::Dispute = dispute::create(oid, txset).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetDisputeResponse::build(m_dispute)),
//     )
// }

// /// Create a dispute
// #[get("/<_address>/<oid>")]
// pub async fn get_dispute(
//     _address: String,
//     _token: auth::BearerToken,
//     oid: String,
// ) -> Custom<Json<reqres::GetDisputeResponse>> {
//     let m_dispute: models::Dispute = dispute::find(oid).await;
//     Custom(
//         Status::Ok,
//         Json(reqres::GetDisputeResponse::build(m_dispute)),
//     )
// }
// END JSON APIs
