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
    Balance,
    Create,
    Close,
    Export,
    Finalize,
    GetVersion,
    Id,
    Import,
    JsonRpcVersion,
    Make,
    Open,
    Prepare,
    SignMultisig,
    Verify,
}

impl RpcFields {
    pub fn value(&self) -> String {
        match *self {
            RpcFields::Balance => String::from("get_balance"),
            RpcFields::Create => String::from("create_wallet"),
            RpcFields::Close => String::from("close_wallet"),
            RpcFields::Export => String::from("export_multisig_info"),
            RpcFields::Finalize => String::from("finalize_multisig"),
            RpcFields::GetVersion => String::from("get_version"),
            RpcFields::Id => String::from("0"),
            RpcFields::Import => String::from("import_multisig_info"),
            RpcFields::JsonRpcVersion => String::from("2.0"),
            RpcFields::Make => String::from("make_multisig"),
            RpcFields::Open => String::from("open_wallet"),
            RpcFields::Prepare => String::from("prepare_multisig"),
            RpcFields::SignMultisig => String::from("sign_multisig"),
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
    let req = reqres::XmrRpcRequest {
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
                _ => Default::default(),
            }
        }
        Err(_e) => Default::default(),
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

/// Performs the xmr rpc 'create_wallet' method
pub async fn create_wallet(filename: String) -> bool {
    info!("creating mulisig wallet for order {}", &filename);
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcWalletParams {
        filename,
        language: String::from("English"),
    };
    let req = reqres::XmrRpcCreateRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Create.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            // The result from wallet creation is empty
            let res = response.text().await;
            debug!("create response: {:?}", res);
            match res {
                Ok(_) => true,
                _ => false,
            }
        }
        Err(_) => false
    }
}

/// Performs the xmr rpc 'open_wallet' method
pub async fn open_wallet(filename: String) -> bool {
    info!("opening wallet for order {}", &filename);
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcWalletParams {
        filename,
        language: String::from("English"),
    };
    let req = reqres::XmrRpcCreateRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Open.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            // The result from wallet operation is empty
            let res = response.text().await;
            debug!("open response: {:?}", res);
            match res {
                Ok(_) => true,
                _ => false,
            }
        }
        Err(_) => false
    }
}

/// Performs the xmr rpc 'close_wallet' method
pub async fn close_wallet(filename: String) -> bool {
    info!("closing wallet for order {}", &filename);
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcWalletParams {
        filename,
        language: String::from("English"),
    };
    let req = reqres::XmrRpcCreateRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Close.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            // The result from wallet operation is empty
            let res = response.text().await;
            debug!("close response: {:?}", res);
            match res {
                Ok(_) => true,
                _ => false,
            }
        }
        Err(_) => false
    }
}

/// Performs the xmr rpc 'get_balance' method
pub async fn get_balance() -> reqres::XmrRpcBalanceResponse {
    info!("fetching wallet balance");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params: reqres::XmrRpcBalanceParams = reqres::XmrRpcBalanceParams { 
        account_index: 0, address_indices: vec![0], all_accounts: false, strict: false, 
    };
    let req = reqres::XmrRpcBalanceRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Balance.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcBalanceResponse>().await;
            debug!("balance response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => Default::default(),
            }
        }
        Err(_) => Default::default()
    }
}
// START Multisig

/// Performs the xmr rpc 'prepare_multisig' method
pub async fn prepare_wallet() -> reqres::XmrRpcPrepareResponse {
    info!("prepare msig wallet");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let req = reqres::XmrRpcRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Prepare.value(),
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcPrepareResponse>().await;
            debug!("prepare response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => Default::default(),
            }
        }
        Err(_) => Default::default()
    }
}

/// Performs the xmr rpc 'make_multisig' method
pub async fn make_wallet(info: Vec<String>) -> reqres::XmrRpcMakeResponse {
    info!("make msig wallet");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcMakeParams {
        multisig_info: info,
        threshold: 2,
    };
    let req = reqres::XmrRpcMakeRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Make.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcMakeResponse>().await;
            debug!("make response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => Default::default(),
            }
        }
        Err(_) => Default::default()
    }
}

/// Performs the xmr rpc 'finalize_multisig' method
pub async fn finalize_wallet(info: Vec<String>) -> reqres::XmrRpcFinalizeResponse {
    info!("finalize msig wallet");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcFinalizeParams {
        multisig_info: info,
    };
    let req = reqres::XmrRpcFinalizeRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Finalize.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcFinalizeResponse>().await;
            debug!("finalize response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => Default::default(),
            }
        }
        Err(_) => Default::default()
    }
}

/// Performs the xmr rpc 'export_multisig_info' method
pub async fn export_multisig_info() -> reqres::XmrRpcExportResponse {
    info!("export msig info");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let req = reqres::XmrRpcRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Export.value(),
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcExportResponse>().await;
            debug!("export msig response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => Default::default(),
            }
        }
        Err(_) => Default::default()
    }
}

/// Performs the xmr rpc 'import_multisig_info' method
pub async fn import_multisig_info(info: Vec<String>) -> reqres::XmrRpcImportResponse {
    info!("import msig wallet");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcImportParams {
        info,
    };
    let req = reqres::XmrRpcImportRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::Import.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcImportResponse>().await;
            debug!("import msig info response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => Default::default(),
            }
        }
        Err(_) => Default::default()
    }
}

/// Performs the xmr rpc 'sign_multisig' method
pub async fn sign_multisig(tx_data_hex: String) -> reqres::XmrRpcSignMultisigResponse {
    info!("sign msig txset");
    let client = reqwest::Client::new();
    let host = get_rpc_host();
    let params = reqres::XmrRpcSignMultisigParams {
        tx_data_hex,
    };
    let req = reqres::XmrRpcSignMultisigRequest {
        jsonrpc: RpcFields::JsonRpcVersion.value(),
        id: RpcFields::Id.value(),
        method: RpcFields::SignMultisig.value(),
        params,
    };
    let login: RpcLogin = get_rpc_creds();
    match client.post(host).json(&req)
    .send_with_digest_auth(&login.username, &login.credential).await {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcSignMultisigResponse>().await;
            debug!("sign msig txset response: {:?}", res);
            match res {
                Ok(res) => res,
                _ => Default::default(),
            }
        }
        Err(_) => Default::default()
    }
}
// END Multisig
