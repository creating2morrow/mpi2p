use crate::auth;
use crate::logger;
use crate::models::*;
use crate::monero;
use crate::schema;
use crate::utils;
use crate::vendor;
use diesel::pg::PgConnection;
use diesel::prelude::*;

async fn create_customer(
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
    diesel::insert_into(customers::table)
        .values(&new_customer)
        .get_result(conn)
        .expect("Error saving new customer")
}

pub async fn find_customer(address: String) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(address))
        .load::<Customer>(connection);
    match results {
        Ok(mut r) => {
            logger::log(logger::LogLevel::INFO, "Found customer.").await;
            if &r.len() > &0 {
                r.remove(0)
            } else {
                Default::default()
            }
        }
        _ => {
            logger::log(logger::LogLevel::ERROR, "Error finding customer.").await;
            Default::default()
        }
    }
}

pub async fn verify_customer_login(address: String, signature: String) -> Authorization {
    let connection = &mut utils::establish_pgdb_connection().await;
    use crate::schema::customers::dsl::*;
    let f_address = String::from(&address);
    let f_auth: Authorization = auth::find_auth(f_address).await;
    let data: String = String::from(&f_auth.rnd);
    if f_auth.xmr_address == String::from("") {
        return auth::create_auth(connection, address).await;
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
                logger::log(logger::LogLevel::INFO, "Creating new customer").await;
                create_customer(connection, &sig_address, "", "").await;
                return f_auth;
            }
        }
        _ => {
            logger::log(logger::LogLevel::ERROR, "Error creating customer.").await;
            Default::default()
        }
    }
}

pub async fn modify_customer(_id: String, data: String, update_type: i32) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    if update_type == vendor::VendorUpdateType::Name.value() {
        logger::log(logger::LogLevel::INFO, "Modify customer name.").await;
        let m = diesel::update(customers.find(_id))
            .set(c_name.eq(data))
            .get_result::<Customer>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == vendor::VendorUpdateType::Pgp.value() {
        logger::log(logger::LogLevel::INFO, "Modify customer PGP.").await;
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
