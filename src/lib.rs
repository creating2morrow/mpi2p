pub mod models;
pub mod schema;
pub mod reqres;
use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// TODO: random data for each login?
const LOGIN_DATA: &str = "LOGIN";
// TODO: Error enum
const GET_XMR_RPC_VERSION_ERROR: &str = "GET_XMR_RPC_VERSION_ERROR";
// TODO: cmd line args
const XMR_RPC_HOST: &str = "http://127.0.0.1:38083/json_rpc";

pub async fn establish_pgdb_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_customer(conn: &mut PgConnection, c_xmr_address: &str, c_name: &str, c_pgp: &str) -> Customer {
    use crate::schema::customers;
    let new_customer = NewCustomer { c_xmr_address, c_name, c_pgp };
    diesel::insert_into(customers::table)
        .values(&new_customer)
        .get_result(conn)
        .expect("Error saving new customer")
}

pub async fn get_xmr_version() -> String {
    let client = reqwest::Client::new();
    let net = XMR_RPC_HOST.to_string(); // TODO: this as cmd line arg
    let req = reqres::XmrRpcVersionRequest { 
        jsonrpc: "2.0".to_string(), 
        id: "0".to_string(), 
        method: "get_version".to_string()
    };
    match client.post(net).json(&req).send().await
    {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVersionResponse>().await;
            match res {
                Ok(res) => format!("{{ \"version\": {} }}", res.result.version),
                _=> GET_XMR_RPC_VERSION_ERROR.to_string()
            }
        }
        Err(_e) => {
            GET_XMR_RPC_VERSION_ERROR.to_string()
        }
    }
}

pub async fn check_xmr_rpc_connection() -> () {
    let ver: String = get_xmr_version().await;
    if ver == GET_XMR_RPC_VERSION_ERROR {
        panic!("Failed to connect to monero-wallet-rpc");
    }
}

pub async fn verify_signature(address: String, signature: String) -> String {
    let client = reqwest::Client::new();
    let net = XMR_RPC_HOST.to_string();
    let params = reqres::XmrRpcVerifyParams {
        address,
        data: LOGIN_DATA.to_string(),
        signature,
    };
    let req = reqres::XmrRpcVerifyRequest { 
        jsonrpc: "2.0".to_string(), 
        id: "0".to_string(), 
        method: "verify".to_string(),
        params,
    };
    match client.post(net).json(&req).send().await
    {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVerifyResponse>().await;
            match res {
                Ok(res) => {
                    if res.result.good {
                        format!("{{ \"address\": {} }}", &req.params.address)
                    } else {
                        GET_XMR_RPC_VERSION_ERROR.to_string()
                    }
                }
                _=> GET_XMR_RPC_VERSION_ERROR.to_string()
            }
        }
        Err(_e) => {
            GET_XMR_RPC_VERSION_ERROR.to_string()
        }
    }
}
