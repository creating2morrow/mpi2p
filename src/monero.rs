use clap::Parser;
use crate::args;
use crate::reqres;
use crate::logger;
use crate::utils;

fn get_monero_rpc_host() -> String {
    let args = args::Args::parse();
    let rpc = String::from(args.monero_rpc_host);
    format!("{}/json_rpc", rpc)
}

enum XmrRpcFields {
    GetVersion,
    Id,
    JsonRpcVersion,
    Verify,
}

impl XmrRpcFields {
    pub fn value(&self) -> String {
        match *self {
            XmrRpcFields::GetVersion => String::from("get_version"),
            XmrRpcFields::Id => String::from("0"),
            XmrRpcFields::JsonRpcVersion => String::from("2.0"),
            XmrRpcFields::Verify => String::from("verify"),
        }
    }
}

pub async fn get_xmr_version() -> reqres::XmrRpcVersionResponse {
    let client = reqwest::Client::new();
    let host = get_monero_rpc_host();
    let req = reqres::XmrRpcVersionRequest {
        jsonrpc: XmrRpcFields::JsonRpcVersion.value(),
        id: XmrRpcFields::Id.value(),
        method: XmrRpcFields::GetVersion.value(),
    };
    match client.post(host).json(&req).send().await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVersionResponse>().await;
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

pub async fn check_xmr_rpc_connection() -> () {
    let res: reqres::XmrRpcVersionResponse = get_xmr_version().await;
    if res.result.version == 0 {
        panic!("Failed to connect to monero-wallet-rpc");
    }
}

pub async fn verify_signature(address: String, data: String, signature: String) -> String {
    logger::log(logger::LogLevel::INFO, "Signature verification in progress.").await;
    let client = reqwest::Client::new();
    let host = get_monero_rpc_host();
    let params = reqres::XmrRpcVerifyParams {
        address,
        data,
        signature,
    };
    let req = reqres::XmrRpcVerifyRequest {
        jsonrpc: XmrRpcFields::JsonRpcVersion.value(),
        id: XmrRpcFields::Id.value(),
        method: XmrRpcFields::Verify.value(),
        params,
    };
    match client.post(host).json(&req).send().await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVerifyResponse>().await;
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
