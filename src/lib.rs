pub mod models;
pub mod schema;
pub mod reqres;
use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

const GET_XMR_RPC_VERSION_ERROR: &str = "GET_XMR_RPC_VERSION_ERROR";

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
    let net = "http://127.0.0.1:38083/json_rpc"; // TODO: this as cmd line arg
    let req = reqres::RpcRequest { 
        jsonrpc: "2.0".to_string(), 
        id: "0".to_string(), 
        method: "get_version".to_string()
    };
    match client
        .post(net)
        .json(&req)
        .send()
        .await
    {
        Ok(response) => {
            let res = response.json::<reqres::RpcResponse>().await;
            match res {
                Ok(res) => format!(
                    "{{ \"version\": {} }}", res.result.version
                    ),
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