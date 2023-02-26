#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use mpi2p::*;

#[cfg(test)]
mod tests;

// JSON APIs

/// Get payment API version
/// Protected: false
#[get("/version")]
async fn get_version() -> Custom<Json<reqres::XmrApiVersionResponse>> {
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
async fn get_customer(
    address: String,
    signature: String,
) -> Custom<Json<reqres::GetCustomerResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        let res: reqres::GetCustomerResponse = Default::default();
        return Custom(Status::Unauthorized, Json(res));
    }
    let m_customer: models::Customer = customer::find_customer(address).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetCustomerResponse::build(m_customer)),
    )
}

/// Get a single vendor's information
/// Protected: true
#[get("/<address>/<signature>")]
async fn get_vendor(address: String, signature: String) -> Custom<Json<reqres::GetVendorResponse>> {
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
async fn login(
    address: String,
    corv: String,
    signature: String,
) -> Custom<Json<reqres::GetAuthResponse>> {
    let m_auth: models::Authorization = auth::get_login_auth(address, corv, signature).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetAuthResponse::build(m_auth)),
    )
}

/// Update customer information
#[patch("/update/<id>/<data>/<update_type>")]
async fn update_customer(
    id: String,
    data: String,
    update_type: i32,
) -> Custom<Json<reqres::GetCustomerResponse>> {
    let m_customer: models::Customer = customer::modify_customer(id, data, update_type).await;
    // TODO: dont pass id, pull info from db after auth
    Custom(
        Status::Accepted,
        Json(reqres::GetCustomerResponse::build(m_customer)),
    )
}

/// Update vendor information
#[patch("/<address>/<signature>/update/<id>/<data>/<update_type>")]
async fn update_vendor(
    address: String,
    signature: String,
    id: String,
    data: String,
    update_type: i32,
) -> Custom<Json<reqres::GetVendorResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    // TODO: dont pass id, pull info from db after auth
    let m_vendor: models::Vendor = vendor::modify_vendor(id, data, update_type).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetVendorResponse::build(m_vendor)),
    )
}

/// Create a product by passing vendor id
#[get("/<address>/<signature>/create/<v_id>")]
async fn create_product(
    address: String,
    signature: String,
    v_id: String,
) -> Custom<Json<reqres::GetProductResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        let res: reqres::GetProductResponse = Default::default();
        return Custom(Status::Unauthorized, Json(res));
    }
    // TODO: dont pass id, pull info from db after auth
    let m_product: models::Product = product::create_new_product(v_id).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetProductResponse::build(m_product)),
    )
}

/// Get all products by passing vendor id
#[get("/<address>/<signature>/<v_id>")]
async fn get_vendor_products(
    address: String,
    signature: String,
    v_id: String,
) -> Custom<Json<reqres::GetVendorProductsResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    // TODO: dont pass vid, pull info from db after auth
    let m_products: Vec<models::Product> = product::find_vendor_products(v_id).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetVendorProductsResponse::build(m_products)),
    )
}

/// Update product information
#[patch("/<address>/<signature>/update/<id>/<data>/<update_type>")]
async fn update_product(
    address: String,
    signature: String,
    id: String,
    data: String,
    update_type: i32,
) -> Custom<Json<reqres::GetProductResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    // TODO: dont pass vid, pull info from db after auth
    let m_product: models::Product = product::modify_product(id, data, update_type).await;
    Custom(
        Status::Accepted,
        Json(reqres::GetProductResponse::build(m_product)),
    )
}

/// Initialize order
#[get("/<address>/<signature>/create/<pid>")]
async fn initialize_order(
    address: String,
    signature: String,
    pid: String,
) -> Custom<Json<reqres::InitializeOrderResponse>> {
    let is_verified: bool = auth::verify_access(&address, &signature).await;
    if !is_verified {
        return Custom(Status::Unauthorized, Json(Default::default()));
    }
    // get the cid from the address after verification
    let m_customer = customer::find_customer(address).await;
    let temp_pid = String::from(&pid);
    let m_order: models::Order = order::create_new_order(m_customer.cid, temp_pid).await;
    Custom(
        Status::Accepted,
        Json(reqres::InitializeOrderResponse::build(pid, m_order)),
    )
}

/// update order

/// Get all orders by passing vendor id

/// Get all orders by passing customer id

// END JSON APIs

#[launch]
async fn rocket() -> _ {
    // pdgb and monero-wallet-rpc are required to be up at boot time
    logger::log(logger::LogLevel::INFO, &("mpi2p is starting up")).await;
    utils::establish_pgdb_connection().await;
    monero::check_xmr_rpc_connection().await;
    let env: String = utils::get_release_env().value();
    if env != utils::ReleaseEnvironment::Development.value() {
        i2p::check_i2p_connection().await;
    }
    logger::log(logger::LogLevel::INFO, &(env + " - mpi2p is online")).await;
    rocket::build()
        .mount("/", routes![login])
        .mount("/customer", routes![get_customer, update_customer])
        .mount("/vendor", routes![get_vendor, update_vendor])
        .mount("/product", routes![create_product, update_product])
        .mount("/products", routes![get_vendor_products])
        .mount("/order", routes![initialize_order])
        .mount("/xmr", routes![get_version])
}
