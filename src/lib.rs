pub mod models;
pub mod schema;
pub mod reqres;

use hex;
use std::env;
use chrono;
use dotenv::dotenv;
use self::models::*;
use clap:: Parser;
use rand_core::RngCore;
use std::fmt::{self, Debug};

use diesel::prelude::*;
use diesel::pg::PgConnection;

// TODO: start refactoring this if it grows larger than 10000 lines

// START Misc. Enumerations
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
pub enum VendorUpdateType {
    Active,
    Description,
    Name,
    Pgp,
}

impl VendorUpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            VendorUpdateType::Active => 0,
            VendorUpdateType::Description => 1,
            VendorUpdateType::Name => 2,
            VendorUpdateType::Pgp => 3,
        }
    }
}

#[derive(Debug)]
pub enum ProductUpdateType {
    InStock,
    Description,
    Name,
    Price,
}

impl ProductUpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            ProductUpdateType::InStock => 0,
            ProductUpdateType::Description => 1,
            ProductUpdateType::Name => 2,
            ProductUpdateType::Price => 3,
        }
    }
}

#[derive(Debug)]
pub enum OrderStatus {
    Delivered,
    Error,
    MultisigMissing,
    MulitsigComplete,
    Signed,
    Shipped,
    Submitted,
}

#[derive(Debug)]
pub enum OrderUpdateType {
    CustomerKex1,
    CustomerKex2,
    CustomerKex3,
    CustomerMultisigInfo,
    Deliver,
    Hash,
    SignMultisig,
    Ship,
    SubmitMultisig,
    Status,
    VendorKex1,
    VendorKex2,
    VendorKex3,
    VendorMultisigInfo,
    Quantity,
}

#[derive(Debug)]
pub enum I2pStatus {
    Accept,
    Reject,
}

impl I2pStatus {
    pub fn value(&self) -> String {
        match *self {
            I2pStatus::Accept => "Accepting tunnels".to_string(),
            I2pStatus::Reject => "Rejecting tunnels: Starting up".to_string(),
        }
    }
}
// END Enumerations

// START logger
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
    /// check i2p
   #[arg(
        short,
        long,
        help = "Disable i2p connection check",
        default_value = "false",
    )]
   disable_i2p_check: bool,
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
   /// Token expiration in minutes
   #[arg(
        short,
        long,
        help = "Set the token expiration limit in minutes.",
        default_value = "60",
   )]
   token_timeout: i64,
}
// END cmd line args

// START PGDB stuff
pub async fn establish_pgdb_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn create_customer
(conn: &mut PgConnection, c_xmr_address: &str, c_name: &str, c_pgp: &str) -> Customer {
    use crate::schema::customers;
    let cid: String = generate_rnd();
    let new_customer = NewCustomer { cid: &cid, c_xmr_address, c_name, c_pgp };
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
            else { Default::default() }
        },
        _=> {
                log(LogLevel::ERROR, "Error finding customer.").await;
                Default::default()
            }
    }
}

async fn create_vendor
(conn: &mut PgConnection, v_xmr_address: &str,
v_name: &str, v_pgp: &str,v_description: &str, active: &bool) -> Vendor {
    use crate::schema::vendors;
    let vid: String = generate_rnd();
    let new_vendor = NewVendor {
        vid: &vid, v_xmr_address, v_name, v_description, v_pgp, active 
    };
    diesel::insert_into(vendors::table)
        .values(&new_vendor)
        .get_result(conn)
        .expect("Error saving new vendor")
}

pub async fn find_vendor(address: String) -> Vendor {
    use self::schema::vendors::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    let results = vendors
        .filter(schema::vendors::v_xmr_address.eq(address))
        .load::<models::Vendor>(connection);
    match results {
        Ok(mut r) => {
            if &r.len() > &0 { 
                log(LogLevel::INFO, "Found vendor.").await;
                r.remove(0) 
            }
            else { Default::default() }
        },
        _=> {
                log(LogLevel::ERROR, "Error finding vendor.").await;
                Default::default()
            }
    }
}

pub async fn create_new_product(v_id: String) -> Product {
    use crate::schema::products;
    let connection = &mut establish_pgdb_connection().await;
    let pid: String = generate_rnd();
    let new_product = NewProduct {
        pid: &pid,
        v_id: &v_id,
        in_stock: &false,
        p_description: "",
        p_name: "",
        p_price: &0,
        qty: &0,
    };
    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(connection)
        .expect("Error saving new product")
}

pub async fn create_new_order(cid: String, pid: String) -> Order {
    use crate::schema::orders;
    let connection = &mut establish_pgdb_connection().await;
    let ts = chrono::offset::Utc::now().timestamp();
    let oid: String = generate_rnd();
    let new_order = NewOrder {
        orid: &oid,
        c_id: &cid,
        p_id: &pid,
        o_cust_kex_1: "",
        o_cust_kex_2: "",
        o_cust_kex_3: "",
        o_xmr_address: "",
        o_cust_msig_info: "",
        o_date: &ts,
        o_deliver_date: &0,
        o_ship_date: &0,
        o_hash: "",
        o_msig_prepare: "",
        o_msig_make: "",
        o_msig_kex_1: "",
        o_msig_kex_2: "",
        o_msig_kex_3: "",
        o_status: "",
        o_quantity: &0,
        o_vend_kex_1: "",
        o_vend_kex_2: "",
        o_vend_kex_3: "",
    };
    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(connection)
        .expect("Error saving new order")
}

pub async fn find_vendor_products(_v_id: String) -> Vec<Product> {
    use self::schema::products::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    let results = products
        .filter(schema::products::v_id.eq(_v_id))
        .load::<models::Product>(connection);
    match results {
        Ok(r) => {
            log(LogLevel::INFO, "Found vendor products.").await;
            r
        },
        _=> {
                log(LogLevel::ERROR, "Error finding vendor products.").await;
                let v: Vec<Product> = Vec::new();
                v
            }
    }
}

pub async fn verify_customer_login(address: String, signature: String) -> Authorization {
    use self::schema::customers::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    let f_address = String::from(&address);
    let f_auth: Authorization = find_auth(f_address).await;
    let data: String = String::from(&f_auth.rnd);
    if f_auth.xmr_address == String::from("") {
        return create_auth(connection, address).await;
    }
    let sig_address: String = verify_signature(address, data, signature).await;
    if sig_address == ApplicationErrors::LoginError.to_string() {
        return f_auth;
    }
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(&sig_address))
        .load::<models::Customer>(connection);
    match results {
        Ok(r) => {
            if &r.len() > &0 {
                return f_auth;
            } else {
                log(LogLevel::INFO, "Creating new customer").await;
                create_customer(connection, &sig_address, "", "").await;
                return f_auth;
            }
        }
        _=> {
            log(LogLevel::ERROR, "Error creating customer.").await;
            Default::default()
        }
    }
}

pub async fn verify_vendor_login(address: String, signature: String) -> Authorization {
    use self::schema::vendors::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    let f_address = String::from(&address);
    let f_auth: Authorization = find_auth(f_address).await;
    let data: String = String::from(&f_auth.rnd);
    if f_auth.xmr_address == String::from("") {
        return create_auth(connection, address).await;
    }
    let sig_address: String = verify_signature(address, data, signature).await;
    if sig_address == ApplicationErrors::LoginError.to_string() {
        return f_auth;
    }
    let results = vendors
        .filter(schema::vendors::v_xmr_address.eq(&sig_address))
        .load::<models::Vendor>(connection);
    match results {
        Ok(r) => {
            if &r.len() > &0 {
                return f_auth;
            } else {
                log(LogLevel::INFO, "Creating new vendor").await;
                create_vendor(connection, &sig_address, "", "", "", &false).await;
                return f_auth;
            }
        }
        _=> {
            log(LogLevel::ERROR, "Error creating vendor.").await;
            Default::default()
        }
    }
}

pub async fn modify_customer(_id: String, data: String, update_type: i32) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    if update_type == VendorUpdateType::Name.value() {
        log(LogLevel::INFO, "Modify customer name.").await;
        let m = diesel::update(customers.find(_id))
            .set(c_name.eq(data))
            .get_result::<Customer>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    else if update_type == VendorUpdateType::Pgp.value() {
        log(LogLevel::INFO, "Modify customer PGP.").await;
        let m = diesel::update(customers.find(_id))
            .set(c_pgp.eq(data))
            .get_result::<Customer>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    Default::default()
}

pub async fn modify_vendor(_id: String, data: String, update_type: i32) -> Vendor {
    use self::schema::vendors::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    if update_type == VendorUpdateType::Active.value() {
        log(LogLevel::INFO, "Modify vendor active status.").await;
        let m = diesel::update(vendors.find(_id))
            .set(active.eq(true))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    else if update_type == VendorUpdateType::Description.value() {
        log(LogLevel::INFO, "Modify vendor description.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_description.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    else if update_type == VendorUpdateType::Name.value() {
        log(LogLevel::INFO, "Modify vendor name.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_name.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    else if update_type == VendorUpdateType::Pgp.value() {
        log(LogLevel::INFO, "Modify vendor pgp.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_pgp.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    Default::default()
}

pub async fn modify_product(_id: String, data: String, update_type: i32) -> Product {
    use self::schema::products::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    // TODO: this isn't right. The product should automatically
    // get updated based on the qty. Qty should be updated according
    // to settled orders
    if update_type == ProductUpdateType::InStock.value() {
        log(LogLevel::INFO, "Modify product active status.").await;
        let m = diesel::update(products.find(_id))
            .set(in_stock.eq(true))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    else if update_type == ProductUpdateType::Description.value() {
        log(LogLevel::INFO, "Modify product description.").await;
        let m = diesel::update(products.find(_id))
            .set(p_description.eq(data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    else if update_type == ProductUpdateType::Name.value() {
        log(LogLevel::INFO, "Modify product name.").await;
        let m = diesel::update(products.find(_id))
            .set(p_name.eq(data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    else if update_type == ProductUpdateType::Price.value() {
        log(LogLevel::INFO, "Modify product price.").await;
        let price_data = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(products.find(_id))
            .set(p_price.eq(price_data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default()
        };
    }
    Default::default()
}

pub async fn create_auth(conn: &mut PgConnection, address:String) -> Authorization {
    use crate::schema::authorizations;
    let aid: String = generate_rnd();
    let rnd: String = generate_rnd();
    let created: i64 = chrono::offset::Utc::now().timestamp();
    let new_auth = NewAuthorization { aid: &aid, created: &created, rnd: &rnd, xmr_address: &address };
    diesel::insert_into(authorizations::table)
        .values(&new_auth)
        .get_result(conn)
        .expect("Error saving new auth")
}

async fn find_auth(address: String) -> Authorization {
    use self::schema::authorizations::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    let results = authorizations
        .filter(schema::authorizations::xmr_address.eq(address))
        .load::<models::Authorization>(connection);
    match results {
        Ok(mut r) => {
            if &r.len() > &0 {
                log(LogLevel::INFO, "Found auth.").await;
                r.remove(0)
            } else { Default::default() }
        },
        _=> {
                log(LogLevel::ERROR, "Error finding auth.").await;
                Default::default()
        }
    }
}

async fn update_auth_expiration(_id: &str) -> Authorization {
    use self::schema::authorizations::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    log(LogLevel::INFO, "Modify auth expiration.").await;
    let time: i64 = chrono::offset::Utc::now().timestamp();
    let m = diesel::update(authorizations.find(_id))
        .set(created.eq(time))
        .get_result::<Authorization>(connection);
    match m {
        Ok(m) => m,
        Err(_e) => Default::default()
    }
}

async fn update_auth_data(_id: &str) -> Authorization {
    use self::schema::authorizations::dsl::*;
    let connection = &mut establish_pgdb_connection().await;
    log(LogLevel::INFO, "Modify auth data.").await;
    let data: String = generate_rnd();
    let m = diesel::update(authorizations.find(_id))
        .set(rnd.eq(data))
        .get_result::<Authorization>(connection);
    match m {
        Ok(m) => m,
        Err(_e) => Default::default()
    }
}

/// TODO: this is a temporary workaround
/// from_request doesn't support async_trait
/// and we need that to verify the authorization header
/// migrate to async from_request impl
pub async fn verify_access(address: &str, signature: &str) -> bool {
    // look up auth for address
    let f_auth: Authorization = find_auth(String::from(address)).await;
    if f_auth.xmr_address != String::from("") {
        // check expiration, generate new data to sign if necessary
        let now: i64 = chrono::offset::Utc::now().timestamp();
        let expiration = get_auth_expiration();
        if now > f_auth.created + expiration {
            update_auth_expiration(&f_auth.aid).await;
            update_auth_data(&f_auth.aid).await;
            return false;
        }
    }
    // verify signature on the data if not expired
    let data = f_auth.rnd;
    let sig_address: String = verify_signature(
        String::from(address), data, String::from(signature)
    ).await;
    if sig_address == ApplicationErrors::LoginError.to_string() {
        return false;
    }
    return true;
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

pub async fn verify_signature(
    address: String, data: String, signature: String
) -> String {
    log(LogLevel::INFO, "Signature verification in progress.").await;
    let client = reqwest::Client::new();
    let host = get_monero_rpc_host();
    let params = reqres::XmrRpcVerifyParams {
        address,
        data,
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

// START i2p connection verification
/// TODO: create a tunnel for the server at initial startup
/// if one does not exist. See i2p-zero
pub async fn check_i2p_connection() -> () {
    let client = reqwest::Client::new();
    let host = "http://localhost:7657/tunnels";
    let tick = schedule_recv::periodic_ms(10000);
    // TODO: better handling and notification of i2p tunnel status
    loop {
        tick.recv().unwrap();
        match client.get(host).send().await
        {
            Ok(response) => {
                // do some parsing here to check the status
                let res = response.text().await;
                match res {
                    Ok(res) => {
                        // split the html from the local i2p tunnels page
                        let split1 = res.split("<h4><span class=\"tunnelBuildStatus\">");
                        let mut v1: Vec<String> = split1.map(|s| s.to_string()).collect();
                        let s1 = v1.remove(1);
                        let v2 = s1.split("</span></h4>");
                        let mut split2: Vec<String> = v2.map(|s| s.to_string()).collect();
                        let status: String = split2.remove(0);
                        if status == I2pStatus::Accept.value() {
                            log(LogLevel::INFO, "I2P is currently accepting tunnels.").await;
                            break;
                        } else if status == I2pStatus::Reject.value() {
                            log(LogLevel::INFO, "I2P is currently rejecting tunnels.").await;
                        } else {
                            log(LogLevel::INFO, "I2P is offline.").await;
                        }
                    },
                    _=> log(LogLevel::ERROR, "I2P status check failure.").await
                }
            }
            Err(_e) => {
                log(LogLevel::ERROR, "I2P status check failure.").await;
            }
        }
    }
}
// END I2P connection verification

// START misc helpers
pub async fn get_login_auth
(address: String, corv: String, signature: String) -> Authorization {
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

fn get_auth_expiration() -> i64 {
    let args = Args::parse();
    args.token_timeout * 60
}

pub fn is_i2p_check_enabled() -> bool {
    let args = Args::parse();
    !args.disable_i2p_check
}

pub fn generate_rnd() -> String {
    let mut data = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);
    hex::encode(data)
}
// END misc. helpers

// START response builders
impl reqres::GetCustomerResponse {
    pub fn build(m_customer: models::Customer) -> Self {
        reqres::GetCustomerResponse {
            cid: m_customer.cid, address: m_customer.c_xmr_address,
            name: m_customer.c_name, pgp: m_customer.c_pgp,
        }
    }
}

impl reqres::GetVendorResponse {
    pub fn build(m_vendor: models::Vendor) -> Self {
        reqres::GetVendorResponse {
            vid: m_vendor.vid, active: m_vendor.active, address: m_vendor.v_xmr_address,
            description: m_vendor.v_description,name: m_vendor.v_name,pgp: m_vendor.v_pgp,
        }
    }
}

impl reqres::GetAuthResponse {
    pub fn build(m_auth: models::Authorization) -> Self {
        reqres::GetAuthResponse {
            address: m_auth.xmr_address, aid: m_auth.aid,
            data: m_auth.rnd, created: m_auth.created,
        }
    }
}

impl reqres::GetProductResponse {
    pub fn build(m_product: models::Product) -> Self {
        reqres::GetProductResponse {
            pid: m_product.pid, v_id: m_product.v_id, in_stock: m_product.in_stock,
            description: m_product.p_description, name: m_product.p_name,
            price: m_product.p_price, qty: m_product.qty,
        }
    }
}

impl reqres::GetVendorProductsResponse {
    pub fn build(m_products: Vec<models::Product>) -> Self {
        let mut v_res: Vec<reqres::GetProductResponse> = Vec::new();
        for m in m_products {
            let p_res: reqres::GetProductResponse = reqres::GetProductResponse {
                pid: m.pid, v_id: m.v_id, in_stock: m.in_stock,
                description: m.p_description, name: m.p_name,
                price: m.p_price, qty: m.qty,
            };
            v_res.push(p_res);
        }
        reqres::GetVendorProductsResponse { products: v_res }
    }
}

impl reqres::InitializeOrderResponse {
    pub fn build(pid: String, m_order: models::Order) -> Self {
        reqres::InitializeOrderResponse {
            orid: m_order.orid, cid: m_order.c_id, pid, xmr_address: m_order.o_xmr_address,
            cust_msig_info: m_order.o_cust_msig_info, cust_kex_1: m_order.o_cust_kex_1,
            cust_kex_2: m_order.o_cust_kex_2, cust_kex_3: m_order.o_cust_kex_3, 
            date: m_order.o_date, deliver_date: m_order.o_deliver_date,
            ship_date: m_order.o_ship_date, hash: m_order.o_hash, 
            msig_prepare: m_order.o_msig_prepare, msig_make: m_order.o_msig_make,
            msig_kex_1: m_order.o_msig_kex_1, msig_kex_2: m_order.o_msig_kex_2,
            msig_kex_3: m_order.o_msig_kex_3, status: m_order.o_status,
            quantity: m_order.o_quantity, vend_kex_1: m_order.o_vend_kex_1,
            vend_kex_2: m_order.o_vend_kex_2, vend_kex_3: m_order.o_vend_kex_3,
        }
    }
}
// END response builders