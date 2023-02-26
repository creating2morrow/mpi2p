use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::monero;
use crate::models::*;
use crate::logger;
use crate::schema;
use crate::utils;
use crate::auth;

#[derive(Debug)]
enum UpdateType {
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

/// Create skeleton for vendor
async fn create(
    conn: &mut PgConnection,
    v_xmr_address: &str,
    v_name: &str,
    v_pgp: &str,
    v_description: &str,
    active: &bool,
) -> Vendor {
    use crate::schema::vendors;
    let vid: String = utils::generate_rnd();
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

/// Verifies vendor signature against stored auth
pub async fn verify_login(address: String, signature: String) -> Authorization {
    use self::schema::vendors::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let f_address = String::from(&address);
    let f_auth: Authorization = auth::find(f_address).await;
    let data: String = String::from(&f_auth.rnd);
    if f_auth.xmr_address == String::from("") {
        return auth::create(connection, address).await;
    }
    let sig_address: String = monero::verify_signature(address, data, signature).await;
    if sig_address == utils::ApplicationErrors::LoginError.value() {
        return f_auth;
    }
    let results = vendors
        .filter(schema::vendors::v_xmr_address.eq(&sig_address))
        .load::<Vendor>(connection);
    match results {
        Ok(r) => {
            if &r.len() > &0 {
                return f_auth;
            } else {
                logger::log(logger::LogLevel::INFO, "Creating new vendor").await;
                create(connection, &sig_address, "", "", "", &false).await;
                return f_auth;
            }
        }
        _ => {
            logger::log(logger::LogLevel::ERROR, "Error creating vendor.").await;
            Default::default()
        }
    }
}

/// Vendor lookup
pub async fn find(address: String) -> Vendor {
    use self::schema::vendors::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let results = vendors
        .filter(schema::vendors::v_xmr_address.eq(address))
        .load::<Vendor>(connection);
    match results {
        Ok(mut r) => {
            if &r.len() > &0 {
                logger::log(logger::LogLevel::INFO, "Found vendor.").await;
                r.remove(0)
            } else {
                Default::default()
            }
        }
        _ => {
            logger::log(logger::LogLevel::ERROR, "Error finding vendor.").await;
            Default::default()
        }
    }
}

/// Update vendor info
pub async fn modify(_id: String, data: String, update_type: i32) -> Vendor {
    use self::schema::vendors::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    if update_type == UpdateType::Active.value() {
        logger::log(logger::LogLevel::INFO, "Modify vendor active status.").await;
        let m = diesel::update(vendors.find(_id))
            .set(active.eq(true))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Description.value() {
        logger::log(logger::LogLevel::INFO, "Modify vendor description.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_description.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Name.value() {
        logger::log(logger::LogLevel::INFO, "Modify vendor name.").await;
        let m = diesel::update(vendors.find(_id))
            .set(v_name.eq(data))
            .get_result::<Vendor>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Pgp.value() {
        logger::log(logger::LogLevel::INFO, "Modify vendor pgp.").await;
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
