use serde::{Deserialize, Serialize};

// This `derive` requires the `serde` dependency.
#[derive(Deserialize, Debug)]
pub struct RpcResult {
    pub version: i32,
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse {
    pub result: RpcResult,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
}