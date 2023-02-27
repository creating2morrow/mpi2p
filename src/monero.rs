use crate::args;
use crate::reqres;
use crate::utils;
use clap::Parser;
use diqwest::WithDigestAuth;
use log::{debug, error, info};

struct RpcLogin {
    username: String,
    credential: String,
}

enum RpcFields {
    GetVersion,
    Id,
    JsonRpcVersion,
    Verify,
}

impl RpcFields {
    pub fn value(&self) -> String {
        match *self {
            RpcFields::GetVersion => String::from("get_version"),
            RpcFields::Id => String::from("0"),
            RpcFields::JsonRpcVersion => String::from("2.0"),
            RpcFields::Verify => String::from("verify"),
        }
    }
}

/// Get monero rpc host from command line argument
fn get_rpc_host() -> String {
    let args = args::Args::parse();
    let rpc = String::from(args.monero_rpc_host);
    format!("{}/json_rpc", rpc)
}

/// Get monero rpc host from command line argument
fn get_rpc_creds() -> RpcLogin {
    let args = args::Args::parse();
    let username = String::from(args.monero_rpc_username);
    let credential = String::from(args.monero_rpc_cred);
    RpcLogin { username, credential }
}

/// Performs rpc 'get_version' method
pub async fn get_version() -> reqres::XmrRpcVersionResponse {
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let req = reqres::XmrRpcVersionRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::GetVersion.value(),
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVersionResponse>().await;
            debug!("get version response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => reqres::XmrRpcVersionResponse {
                    result: reqres::XmrRpcVersionResult { version: 0 },
                },
            }
        }
        Err(_e) => reqres::XmrRpcVersionResponse {
            result: reqres::XmrRpcVersionResult { version: 0 },
        },
    }
}

/// Helper function for checking xmr rpc online during app startup
pub async fn check_rpc_connection() -> () {
    let res: reqres::XmrRpcVersionResponse = get_version().await;
    if res.result.version == 0 {
        error!("failed to connect to monero-wallet-rpc");
    }
}

/// Performs the xmr rpc 'verify' method
pub async fn verify_signature(address: String, data: String, signature: String) -> String {
    info!("signature verification in progress");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcVerifyParams {
        address,
        data,
        signature,
    };
    let req = reqres::XmrRpcVerifyRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Verify.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVerifyResponse>().await;
            debug!("verify response: {:?}", res);
            match res {
                Ok(res) => {
                    if res.result.good {
                        req.params.address
                    } else {
                        utils::ApplicationErrors::LoginError.value()
                    }
                }
                _ => utils::ApplicationErrors::LoginError.value(),
            }
        }
        Err(_e) => utils::ApplicationErrors::LoginError.value(),
    }
}

// START Multisig

// END Multisig
