pub mod args;
pub mod i2p;
pub mod logger;
pub mod models;
pub mod monero;
pub mod reqres;
pub mod schema;
pub mod utils;

use self::models::*;
use clap::Parser;
use hex;
use rand_core::RngCore;
use std::fmt::Debug;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::logger::{log, LogLevel};
use crate::monero::*;
use crate::utils::ApplicationErrors;

// START Misc. Enumerations
#[derive(Debug)]
pub enum ReleaseEnvironment {
    Development,
    Production,
}

impl ReleaseEnvironment {
    pub fn value(&self) -> String {
        match *self {
            ReleaseEnvironment::Development => String::from("development"),
            ReleaseEnvironment::Production => String::from("production"),
        }
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
            LoginType::Customer => String::from("customer"),
            LoginType::Vendor => String::from("vendor"),
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
pub enum OrderStatusType {
    Delivered,
    Error,
    MultisigMissing,
    MulitsigComplete,
    Signed,
    Shipped,
    Submitted,
}

impl OrderStatusType {
    pub fn value(&self) -> i32 {
        match *self {
            OrderStatusType::Delivered => 0,
            OrderStatusType::Error => 1,
            OrderStatusType::MultisigMissing => 2,
            OrderStatusType::MulitsigComplete => 3,
            OrderStatusType::Signed => 4,
            OrderStatusType::Shipped => 5,
            OrderStatusType::Submitted => 6,
        }
    }
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
// END Enumerations

// START PGDB stuff
pub async fn establish_pgdb_connection() -> PgConnection {
    let args = args::Args::parse();
    let db_string: String = String::from(args.postgres_db_url);
    PgConnection::establish(&db_string)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_string))
}

async fn create_customer(
    conn: &mut PgConnection,
    c_xmr_address: &str,
    c_name: &str,
    c_pgp: &str,
) -> Customer {
    use crate::schema::customers;
    let cid: String = generate_rnd();
    let new_customer = NewCustomer {
        cid: &cid,
        c_xmr_address,
        c_name,
        c_pgp,
    };
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
            if &r.len() > &0 {
                r.remove(0)
            } else {
                Default::default()
            }
        }
        _ => {
            log(LogLevel::ERROR, "Error finding customer.").await;
            Default::default()
        }
    }
}

async fn create_vendor(
    conn: &mut PgConnection,
    v_xmr_address: &str,
    v_name: &str,
    v_pgp: &str,
    v_description: &str,
    active: &bool,
) -> Vendor {
    use crate::schema::vendors;
    let vid: String = generate_rnd();
    let new_vendor = NewVendor {
        vid: &vid,
        v_xmr_address,
        v_name,
        v_description,
        v_pgp,
        active,
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
            } else {
                Default::default()
            }
        }
        _ => {
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
        }
        _ => {
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
    if sig_address == ApplicationErrors::LoginError.value() {
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
        _ => {
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
    if sig_address == ApplicationErrors::LoginError.value() {
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
        _ => {
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
            Err(_e) => Default::default(),
        };
    } else if update_type == VendorUpdateType::Pgp.value() {
        log(LogLevel::INFO, "Modify customer PGP.").await;
        let m = diesel::update(customers.find(_id))
            .set(c_pgp.eq(data))
            .get_result::<Customer>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
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
            Err(_e) => Default::default(),
        };
    } else if update_type == VendorUpdateType::Description.value() {
        log(LogLevel::INFO, "Modify vendor description.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_description.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == VendorUpdateType::Name.value() {
        log(LogLevel::INFO, "Modify vendor name.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_name.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == VendorUpdateType::Pgp.value() {
        log(LogLevel::INFO, "Modify vendor pgp.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_pgp.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
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
            Err(_e) => Default::default(),
        };
    } else if update_type == ProductUpdateType::Description.value() {
        log(LogLevel::INFO, "Modify product description.").await;
        let m = diesel::update(products.find(_id))
            .set(p_description.eq(data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == ProductUpdateType::Name.value() {
        log(LogLevel::INFO, "Modify product name.").await;
        let m = diesel::update(products.find(_id))
            .set(p_name.eq(data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == ProductUpdateType::Price.value() {
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
            Err(_e) => Default::default(),
        };
    }
    Default::default()
}

pub async fn create_auth(conn: &mut PgConnection, address: String) -> Authorization {
    use crate::schema::authorizations;
    let aid: String = generate_rnd();
    let rnd: String = generate_rnd();
    let created: i64 = chrono::offset::Utc::now().timestamp();
    let new_auth = NewAuthorization {
        aid: &aid,
        created: &created,
        rnd: &rnd,
        xmr_address: &address,
    };
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
            } else {
                Default::default()
            }
        }
        _ => {
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
        Err(_e) => Default::default(),
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
        Err(_e) => Default::default(),
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
    let sig_address: String =
        verify_signature(String::from(address), data, String::from(signature)).await;
    if sig_address == ApplicationErrors::LoginError.value() {
        return false;
    }
    return true;
}
// END PGDB stuff

// START misc helpers
pub async fn get_login_auth(address: String, corv: String, signature: String) -> Authorization {
    if corv == LoginType::Customer.value() {
        verify_customer_login(address, signature).await
    } else {
        verify_vendor_login(address, signature).await
    }
}

pub fn get_release_env() -> ReleaseEnvironment {
    let args = args::Args::parse();
    let env = String::from(args.release_env);
    if env == "prod" {
        return ReleaseEnvironment::Production;
    } else {
        return ReleaseEnvironment::Development;
    }
}

fn get_auth_expiration() -> i64 {
    let args = args::Args::parse();
    args.token_timeout * 60
}

pub fn generate_rnd() -> String {
    let mut data = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);
    hex::encode(data)
}
// END misc. helpers
