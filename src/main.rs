#[macro_use] extern crate rocket;
use rocket::serde::json::{Json};
use rocket::response::status::Custom;
use rocket::http::Status;

use mpi2p::*;

#[cfg(test)] mod tests;

/*
 TODO:
   - signature verification on update apis !!!
   - create_order
   - get_order/(s) * vendor or customer
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
        cid: m_customer.cid, address: m_customer.c_xmr_address,
        name: m_customer.c_name, pgp: m_customer.c_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Get a single vendor's information
#[get("/<address>")]
async fn get_vendor(address: String) -> Custom<Json<reqres::GetVendorResponse>> {
    let m_vendor: models::Vendor = find_vendor(address).await;
    let res: reqres::GetVendorResponse = reqres::GetVendorResponse {
        vid: m_vendor.vid, active: m_vendor.active, address: m_vendor.v_xmr_address,
        description: m_vendor.v_description,
        name: m_vendor.v_name,pgp: m_vendor.v_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Login with wallet signature
#[get("/login/<corv>/<address>/<data>/<signature>")]
async fn login(address: String, corv: String, data: String, signature: String) -> Custom<Json<reqres::GetAuthResponse>> {
    let m_auth: models::Authorization = get_login_auth(address, corv, data, signature).await;
    let res: reqres::GetAuthResponse = reqres::GetAuthResponse {
        address: m_auth.xmr_address, aid: m_auth.aid, data: m_auth.rnd, created: m_auth.created,
    };
    // TODO: return 401 on bad auth and calculate expiration
    // if r_address == ApplicationErrors::LoginError.to_string() {
    //     Custom(Status::BadRequest, Json(badreq))
    // } else {
    //     Custom(Status::Accepted, Json(reqres::XmrApiVerifyResponse { address: r_address }))
    // }
    Custom(Status::Accepted, Json(res))
}

/// Update customer information
#[patch("/update/<id>/<data>/<update_type>")]
async fn update_customer(id: String, data: String, update_type: i32) -> Custom<Json<reqres::GetCustomerResponse>> {
    let m_customer: models::Customer = modify_customer(id, data, update_type).await;
    let res: reqres::GetCustomerResponse = reqres::GetCustomerResponse {
        cid: m_customer.cid, address: m_customer.c_xmr_address,
        name: m_customer.c_name, pgp: m_customer.c_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Update vendor information
#[patch("/update/<id>/<data>/<update_type>")]
async fn update_vendor(id: String, data: String, update_type: i32) -> Custom<Json<reqres::GetVendorResponse>> {
    let m_vendor = modify_vendor(id, data, update_type).await;
    let res: reqres::GetVendorResponse = reqres::GetVendorResponse {
       vid: m_vendor.vid, active: m_vendor.active, address: m_vendor.v_xmr_address,
        description: m_vendor.v_description, name: m_vendor.v_name, pgp: m_vendor.v_pgp,
    };
    Custom(Status::Accepted, Json(res))
}

/// Create a product by passing vendor id
#[get("/create/<v_id>")]
async fn create_product(v_id: String) -> Custom<Json<reqres::GetProductResponse>> {
    let m_product: models::Product = create_new_product(v_id).await;
    let res: reqres::GetProductResponse = reqres::GetProductResponse {
        pid: m_product.pid, v_id: m_product.v_id, in_stock: m_product.in_stock,
        description: m_product.p_description, name: m_product.p_name,
        price: m_product.p_price, qty: m_product.qty,
    };
    Custom(Status::Accepted, Json(res))
}

/// Get all products by passing vendor id
#[get("/<v_id>")]
async fn get_vendor_products(v_id: String) -> Custom<Json<reqres::GetVendorProductResponse>> {
    let m_products: Vec<models::Product> = find_vendor_products(v_id).await;
    let mut v_res: Vec<reqres::GetProductResponse> = Vec::new();
    // TODO: why cant the db query be serialized and returned?
    for m in m_products {
        let p_res: reqres::GetProductResponse = reqres::GetProductResponse {
            pid: m.pid, v_id: m.v_id, in_stock: m.in_stock,
            description: m.p_description, name: m.p_name,
            price: m.p_price, qty: m.qty,
        };
        v_res.push(p_res);
    }
    Custom(Status::Accepted, Json(reqres::GetVendorProductResponse { products: v_res }))
}

/// Update product information
#[patch("/update/<id>/<data>/<update_type>")]
async fn update_product(
    id: String, data: String, update_type: i32
) -> Custom<Json<reqres::GetProductResponse>> {
    let m_product = modify_product(id, data, update_type).await;
    let res: reqres::GetProductResponse = reqres::GetProductResponse {
        pid: m_product.pid, v_id: m_product.v_id, in_stock: m_product.in_stock,
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
        .mount("/product", routes![create_product, update_product])
        .mount("/products", routes![get_vendor_products])
        // .mount("/order", routes![get_order, update_order])
        .mount("/xmr", routes![get_version])
}
