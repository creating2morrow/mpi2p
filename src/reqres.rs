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
