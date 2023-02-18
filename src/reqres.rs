use serde::{Deserialize, Serialize};
// All http requests and responses are here

#[derive(Deserialize, Debug)]
pub struct XmrRpcVerifyResult {
    pub good: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct XmrApiVerifyResponse {
    pub address: String,
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
    pub id: i32,
    pub name: String,
    pub pgp: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVendorResponse {
    pub active: bool,
    pub address: String,
    pub description: String,
    pub id: i32,
    pub name: String,
    pub pgp: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetProductResponse {
    pub id: i32,
    pub v_id: i32,
    pub in_stock: bool,
    pub description: String,
    pub name: String,
    pub price: i32,
    pub qty: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVendorProductResponse {
    pub products: Vec<GetProductResponse>,
}
