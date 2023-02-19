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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVendorProductResponse {
    pub products: Vec<GetProductResponse>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAuthResponse {
    pub address: String,
    pub aid: String,
    pub data: String,
    pub expires: i64,
}
