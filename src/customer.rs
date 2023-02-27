// Customer repo/service layer
use crate::auth;
use crate::logger;
use crate::models::*;
use crate::monero;
use crate::schema;
use crate::utils;
use diesel::pg::PgConnection;
use diesel::prelude::*;

enum UpdateType {
    Name,
    Pgp,
}

impl UpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            UpdateType::Name => 2,
            UpdateType::Pgp => 3,
        }
    }
}

/// Create a new customer
async fn create(
    conn: &mut PgConnection,
    xmr_address: &str,
    name: &str,
    pgp: &str,
) -> Customer {
    use crate::schema::customers;
    let s_cid: String = utils::generate_rnd();
    let new_customer = NewCustomer {
        cid: &s_cid,
        c_xmr_address: xmr_address,
        c_name: name,
        c_pgp: pgp,
    };
    logger::Log::debug(&format!("insert customer: {:?}", new_customer)).await;
    diesel::insert_into(customers::table)
        .values(&new_customer)
        .get_result(conn)
        .expect("error saving new customer")
}

/// Lookup customer
pub async fn find(address: String) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(address))
        .load::<Customer>(connection);
    match results {
        Ok(mut r) => {
            logger::Log::info("found customer").await;
            if &r.len() > &0 {
                r.remove(0)
            } else {
                Default::default()
            }
        }
        _ => {
            logger::Log::error("error finding customer").await;
            Default::default()
        }
    }
}

/// Performs the signature verfication against stored auth
pub async fn verify_login(address: String, signature: String) -> Authorization {
    let connection = &mut utils::establish_pgdb_connection().await;
    use crate::schema::customers::dsl::*;
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
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(&sig_address))
        .load::<Customer>(connection);
    match results {
        Ok(r) => {
            if &r.len() > &0 {
                return f_auth;
            } else {
                logger::Log::info("creating new customer").await;
                create(connection, &sig_address, "", "").await;
                return f_auth;
            }
        }
        _ => {
            logger::Log::error("error creating customer").await;
            Default::default()
        }
    }
}


/// Update customer information
pub async fn modify(_id: String, data: String, update_type: i32) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    if update_type == UpdateType::Name.value() {
        logger::Log::info("modify customer name").await;
        let m = diesel::update(customers.find(_id))
            .set(c_name.eq(data))
            .get_result::<Customer>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        }
    } else if update_type == UpdateType::Pgp.value() {
        logger::Log::info("modify customer pgp").await;
        let m = diesel::update(customers.find(_id))
            .set(c_pgp.eq(data))
            .get_result::<Customer>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        }
    }
    Default::default()
}
