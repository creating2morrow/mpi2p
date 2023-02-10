pub mod models;
pub mod schema;
pub mod reqres;
use self::models::*;
use std::fmt::{self, Debug};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

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

// TODO: cmd line args
const XMR_RPC_HOST: &str = "http://127.0.0.1:38083/json_rpc";

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
                println!("Creating new customer");
                create_customer(connection, &sig_address, "", "").await;
                sig_address.to_string()
            }
        }
        _=> {
            println!("Error creating customer.");
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
                println!("Creating new vendor");
                create_vendor(connection, &sig_address, "", "", "", &false).await;
                sig_address.to_string()
            }
        }
        _=> {
            println!("Error creating vendor.");
            ApplicationErrors::CreateVendorError.to_string()
        }
    }
}
// END PGDB stuff

// XMR RPC stuff
pub async fn get_xmr_version() -> String {
    let client = reqwest::Client::new();
    let net = XMR_RPC_HOST.to_string();
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
