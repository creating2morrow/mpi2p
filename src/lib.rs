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
pub enum ApplicationErrors {
    LoginError,
}

impl fmt::Display for ApplicationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum LoginType {
    Customer,
    Vendor,
}

impl LoginType {
    pub fn value(&self) -> String {
        match *self {
            LoginType::Customer => "customer".to_string(),
            LoginType::Vendor => "vendor".to_string(),
        }
    }
}


#[derive(Debug)]
pub enum UpdateType {
    Active,
    Description,
    Name,
    Pgp,
}

impl UpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            UpdateType::Active => 0,
            UpdateType::Description => 1,
            UpdateType::Name => 2,
            UpdateType::Pgp => 3,
        }
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
            "{}", format!("|{:?}|\t|{:?}| => {}", level, chrono::offset::Utc::now(), msg)
        );
    }
}
// END logger stuff

// cmd line args
#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Log Level
   #[arg(
        short,
        long,
        help = "Comma separated log level e.g. <WARN,INFO...>",
        default_value = "ERROR,INFO",
    )]
   log_level: String,
   /// Monero RPC HOST
   #[arg(
        short,
        long,
        help = "Monero RPC host.",
        default_value = "http://localhost:38083",
   )]
   monero_rpc_host: String,
}
// end cmd line args

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

pub async fn find_customer(address: String) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(address))
        .load::<models::Customer>(connection);
    match results {
        Ok(mut r) => {
            log(LogLevel::INFO, "Found customer.").await;
            if &r.len() > &0 { r.remove(0) }
            else { get_default_customer() }
        },
        _=> {
                log(LogLevel::ERROR, "Error finding customer.").await;
                get_default_customer()
            }
    }
}

pub async fn find_vendor(address: String) -> Vendor {
    use self::schema::vendors::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    let results = vendors
        .filter(schema::vendors::v_xmr_address.eq(address))
        .load::<models::Vendor>(connection);
    match results {
        Ok(mut r) => {
            log(LogLevel::INFO, "Found vendor.").await;
            if &r.len() > &0 { r.remove(0) }
            else { get_default_vendor() }
        },
        _=> {
                log(LogLevel::ERROR, "Error finding vendor.").await;
                get_default_vendor()
            }
    }
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
    if sig_address == ApplicationErrors::LoginError.to_string() {
        return sig_address;
    }
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
            ApplicationErrors::LoginError.to_string()
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
            ApplicationErrors::LoginError.to_string()
        }
    }
}

pub async fn modify_customer(_id: i32, data: String, update_type: i32) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    if update_type == UpdateType::Name.value() {
        log(LogLevel::INFO, "Modify customer name.").await;
        let m = diesel::update(customers.find(_id))
            .set(c_name.eq(data))
            .get_result::<Customer>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => get_default_customer()
        };
    }
    else if update_type == UpdateType::Pgp.value() {
        log(LogLevel::INFO, "Modify customer PGP.").await;
        let m = diesel::update(customers.find(id))
            .set(c_pgp.eq(data))
            .get_result::<Customer>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => get_default_customer()
        };
    }
    get_default_customer()
}

pub async fn modify_vendor(_id: i32, data: String, update_type: i32) -> Vendor {
    use self::schema::vendors::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    if update_type == UpdateType::Active.value() {
        log(LogLevel::INFO, "Modify vendor active status.").await;
        let m = diesel::update(vendors.find(_id))
            .set(active.eq(true))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => get_default_vendor()
        };
    }
    else if update_type == UpdateType::Description.value() {
        log(LogLevel::INFO, "Modify vendor description.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_description.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => get_default_vendor()
        };
    }
    else if update_type == UpdateType::Name.value() {
        log(LogLevel::INFO, "Modify vendor name.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_name.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => get_default_vendor()
        };
    }
    else if update_type == UpdateType::Pgp.value() {
        log(LogLevel::INFO, "Modify vendor pgp.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_pgp.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => get_default_vendor()
        };
    }
    get_default_vendor()
}
// END PGDB stuff

// XMR RPC stuff
pub async fn get_xmr_version() -> reqres::XmrRpcVersionResponse {
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
                Ok(res) => res,
                _=> reqres::XmrRpcVersionResponse { 
                    result: reqres::XmrRpcVersionResult { version: 0 } 
                }
            }
        }
        Err(_e) => {
            reqres::XmrRpcVersionResponse { 
                result: reqres::XmrRpcVersionResult { version: 0 } 
            }
        }
    }
}

pub async fn check_xmr_rpc_connection() -> () {
    let res: reqres::XmrRpcVersionResponse = get_xmr_version().await;
    if res.result.version == 0 {
        panic!("Failed to connect to monero-wallet-rpc");
    }
}

pub async fn verify_signature(address: String, signature: String) -> String {
    log(LogLevel::INFO, "Signature verification in progress.").await;
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
                        req.params.address
                    } else {
                        ApplicationErrors::LoginError.to_string()
                    }
                }
                _=> ApplicationErrors::LoginError.to_string()
            }
        }
        Err(_e) => {
            ApplicationErrors::LoginError.to_string()
        }
    }
}
// END XMR RPC stuff

// i2p connection verification
pub async fn check_i2p_connection() -> () {
    let client = reqwest::Client::new();
    let host = "http://localhost:7657/tunnels";
    match client.get(host).send().await
    {
        Ok(response) => {
            // do some parsing here to check the status
            let res = response.text().await;
            match res {
                Ok(res) => log(LogLevel::ERROR, &res).await,
                _=> log(LogLevel::ERROR, "I2P status check failure.").await
            }
        }
        Err(_e) => {
            log(LogLevel::ERROR, "I2P status check failure.").await;
        }
    }
}
// END I2P connection verification

// misc helpers
pub async fn get_login_address(address: String, corv: String, signature: String) -> String {
    if corv == LoginType::Customer.value() {
        verify_customer_login(address, signature).await
    } else {
        verify_vendor_login(address, signature).await
    }
}

fn get_monero_rpc_host() -> String {
    let args = Args::parse();
    let rpc = args.monero_rpc_host.to_string();
    format!("{}/json_rpc", rpc)
}

fn get_default_customer() -> Customer {
    Customer { 
        id: 0,
        c_xmr_address: "".to_string(),
        c_name: "".to_string(),
        c_pgp: "".to_string(),
    }
}

fn get_default_vendor() -> Vendor {
    Vendor { 
        id: 0,
        v_xmr_address: "".to_string(),
        v_name: "".to_string(),
        v_description: "".to_string(),
        v_pgp: "".to_string(),
        active: false,
    }
}
// END misc. helpers