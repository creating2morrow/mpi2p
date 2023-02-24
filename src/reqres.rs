use serde::{Deserialize, Serialize};

// All http requests and responses are here

#[derive(Deserialize, Debug)]
pub struct XmrRpcVerifyResult {
    pub good: bool,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcVerifyResponse {
    pub result: XmrRpcVerifyResult,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcVerifyParams {
    pub address: String,
    pub data: String,
    pub signature: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcVerifyRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcVerifyParams,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcVersionResult {
    pub version: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct XmrApiVersionResponse {
    pub version: i32,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcVersionResponse {
    pub result: XmrRpcVersionResult,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcVersionRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetCustomerResponse {
    pub address: String,
    pub cid: String,
    pub name: String,
    pub pgp: String,
}

impl Default for GetCustomerResponse {
    fn default() -> Self {
        GetCustomerResponse {
            address: String::from(""),
            cid: String::from(""),
            name: String::from(""),
            pgp: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVendorResponse {
    pub active: bool,
    pub address: String,
    pub description: String,
    pub vid: String,
    pub name: String,
    pub pgp: String,
}

impl Default for GetVendorResponse {
    fn default() -> Self {
        GetVendorResponse {
            active: false,
            address: String::from(""),
            description: String::from(""),
            vid: String::from(""),
            name: String::from(""),
            pgp: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetProductResponse {
    pub pid: String,
    pub v_id: String,
    pub in_stock: bool,
    pub description: String,
    pub name: String,
    pub price: i64,
    pub qty: i64,
}

impl Default for GetProductResponse {
    fn default() -> Self {
        GetProductResponse {
            pid: String::from(""),
            v_id: String::from(""),
            in_stock: false,
            description: String::from(""),
            name: String::from(""),
            price: 0,
            qty: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVendorProductResponse {
    pub products: Vec<GetProductResponse>,
}

impl Default for GetVendorProductResponse {
    fn default() -> Self {
        GetVendorProductResponse {
            products: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAuthResponse {
    pub address: String,
    pub aid: String,
    pub created: i64,
    pub data: String,
}

impl Default for GetAuthResponse {
    fn default() -> Self {
        GetAuthResponse {
            address: String::from(""),
            aid: String::from(""),
            created: 0,
            data: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InitializeOrderResponse {
    pub orid: String,
    pub cid: String,
    pub pid: String,
    pub xmr_address: String,
    pub cust_msig_info: String,
    pub cust_kex_1: String,
    pub cust_kex_2: String,
    pub cust_kex_3: String,
    pub date: i64,
    pub deliver_date: i64,
    pub ship_date: i64,
    pub hash: String,
    pub msig_prepare: String,
    pub msig_make: String,
    pub msig_kex_1: String,
    pub msig_kex_2: String,
    pub msig_kex_3: String,
    pub status: String,
    pub quantity: i64,
    pub vend_kex_1: String,
    pub vend_kex_2: String,
    pub vend_kex_3: String,
}

impl Default for InitializeOrderResponse {
    fn default() -> Self {
        InitializeOrderResponse {
            orid: String::from(""),
            cid: String::from(""),
            pid: String::from(""),
            xmr_address: String::from(""),
            cust_msig_info: String::from(""),
            cust_kex_1: String::from(""),
            cust_kex_2: String::from(""),
            cust_kex_3: String::from(""),
            date: 0,
            deliver_date: 0,
            ship_date: 0,
            hash: String::from(""),
            msig_prepare: String::from(""),
            msig_make: String::from(""),
            msig_kex_1: String::from(""),
            msig_kex_2: String::from(""),
            msig_kex_3: String::from(""),
            status: String::from(""),
            quantity: 0,
            vend_kex_1: String::from(""),
            vend_kex_2: String::from(""),
            vend_kex_3: String::from(""),
        }
    }
}
