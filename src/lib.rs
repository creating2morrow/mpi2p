pub mod models;
pub mod schema;
pub mod reqres;

use std::env;
use chrono;
use dotenv::dotenv;
use self::models::*;
use clap:: Parser;
use diesel::prelude::*;
use std::fmt::{self, Debug};
use diesel::pg::PgConnection;

// TODO: random data for each login?
const LOGIN_DATA: &str = "LOGIN";

#[derive(Debug)]
enum ApplicationErrors {
    CreateCustomerError,
    CreateVendorError,
    XmrRpcVersionError,
    XmrVerifyError,
}

impl fmt::Display for ApplicationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// logger
#[derive(Debug, Clone)]
pub enum LogLevel {
    DEBUG,
    ERROR,
    INFO,
    WARN,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub async fn log(level: LogLevel, msg: &str) -> () {
    let args = Args::parse();
    let set_level = args.log_level.split(",");
    let vec: Vec<String> = set_level.map(|s| s.to_string()).collect();
    if vec.contains(&level.to_string()) {
        println!(
            "{}", format!("|{:?}| |{:?}| =>  {}", level, chrono::offset::Utc::now(), msg)
        );
    }
}
// END logger stuff

// cmd line args
#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Monero RPC HOST
   #[arg(short, long)]
   monero_rpc_host: String,
   /// Log Level
   #[arg(short,
        long,
        help = "Comma separated log level e.g. <WARN,INFO...>",
        default_value = "ERROR,INFO",
    )]
   log_level: String
}
// end cmd line args

fn get_monero_rpc_host () -> String {
    let args = Args::parse();
    let rpc = args.monero_rpc_host.to_string();
    format!("{}/json_rpc", rpc)
}

// PGDB stuff
pub async fn establish_pgdb_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn create_customer(
    conn: &mut PgConnection, c_xmr_address: &str, c_name: &str, c_pgp: &str
) -> Customer {
    use crate::schema::customers;
    let new_customer = NewCustomer { c_xmr_address, c_name, c_pgp };
    diesel::insert_into(customers::table)
        .values(&new_customer)
        .get_result(conn)
        .expect("Error saving new customer")
}

async fn create_vendor(
    conn: &mut PgConnection, v_xmr_address: &str, v_name: &str, v_pgp: &str,
    v_description: &str, active: &bool
) -> Vendor {
    use crate::schema::vendors;
    let new_vendor = NewVendor {
        v_xmr_address, v_name, v_description, v_pgp, active 
    };
    diesel::insert_into(vendors::table)
        .values(&new_vendor)
        .get_result(conn)
        .expect("Error saving new vendor")
}

pub async fn verify_customer_login(address: String, signature: String) -> String {
    use self::schema::customers::dsl::*;
    let sig_address: String = verify_signature(address, signature).await;
    let connection = &mut establish_pgdb_connection().await;
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(&sig_address))
        .load::<models::Customer>(connection);
    match results {
        Ok(r) => {
            if &r.len() > &0 {
                let result: &str = &r[0].c_xmr_address;
                result.to_string()
            } else {
                log(LogLevel::INFO, "Creating new customer").await;
                create_customer(connection, &sig_address, "", "").await;
                sig_address.to_string()
            }
        }
        _=> {
            log(LogLevel::ERROR, "Error creating customer.").await;
            ApplicationErrors::CreateCustomerError.to_string()
        }
    }
}

pub async fn verify_vendor_login(address: String, signature: String) -> String {
    use self::schema::vendors::dsl::*;
    let sig_address: String = verify_signature(address, signature).await;
    let connection = &mut establish_pgdb_connection().await;
    let results = vendors
        .filter(schema::vendors::v_xmr_address.eq(&sig_address))
        .load::<models::Vendor>(connection);
    match results {
        Ok(r) => {
            if &r.len() > &0 {
                let result: &str = &r[0].v_xmr_address;
                result.to_string()
            } else {
                log(LogLevel::INFO, "Creating new vendor").await;
                create_vendor(connection, &sig_address, "", "", "", &false).await;
                sig_address.to_string()
            }
        }
        _=> {
            log(LogLevel::ERROR, "Error creating vendor.").await;
            ApplicationErrors::CreateVendorError.to_string()
        }
    }
}
// END PGDB stuff

// XMR RPC stuff
pub async fn get_xmr_version() -> String {
    let client = reqwest::Client::new();
    let host = get_monero_rpc_host();
    let req = reqres::XmrRpcVersionRequest { 
        jsonrpc: "2.0".to_string(), 
        id: "0".to_string(), 
        method: "get_version".to_string()
    };
    match client.post(host).json(&req).send().await
    {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVersionResponse>().await;
            match res {
                Ok(res) => format!("{{ \"version\": {} }}", res.result.version),
                _=> ApplicationErrors::XmrRpcVersionError.to_string()
            }
        }
        Err(_e) => {
            ApplicationErrors::XmrRpcVersionError.to_string()
        }
    }
}

pub async fn check_xmr_rpc_connection() -> () {
    let ver: String = get_xmr_version().await;
    if ver == ApplicationErrors::XmrRpcVersionError.to_string() {
        panic!("Failed to connect to monero-wallet-rpc");
    }
}

pub async fn verify_signature(address: String, signature: String) -> String {
    let client = reqwest::Client::new();
    let host = get_monero_rpc_host();
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
    match client.post(host).json(&req).send().await
    {
        Ok(response) => {
            let res = response.json::<reqres::XmrRpcVerifyResponse>().await;
            match res {
                Ok(res) => {
                    if res.result.good {
                        format!("{{ \"address\": {} }}", &req.params.address)
                    } else {
                        ApplicationErrors::XmrVerifyError.to_string()
                    }
                }
                _=> ApplicationErrors::XmrVerifyError.to_string()
            }
        }
        Err(_e) => {
            ApplicationErrors::XmrVerifyError.to_string()
        }
    }
}
// END XMR RPC stuff
