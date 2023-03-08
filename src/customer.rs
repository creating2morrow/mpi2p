// Customer repo/service layer
use crate::auth;
use crate::auth::verify_access;
use crate::models::*;
use crate::monero;
use crate::schema;
use crate::utils;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::{debug, error, info};
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
    let f_cid: String = format!("C{}", utils::generate_rnd());
    let new_customer = NewCustomer {
        cid: &f_cid,
        c_xmr_address: xmr_address,
        c_name: name,
        c_pgp: pgp,
    };
    debug!("insert customer: {:?}", new_customer);
    diesel::insert_into(customers::table)
        .values(&new_customer)
        .get_result(conn)
        .expect("error saving new customer")
}

/// Lookup customer
pub async fn find(address: String) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let result = customers
        .filter(schema::customers::c_xmr_address.eq(address))
        .first::<Customer>(connection);
    match result {
        Ok(r) => r,
        _ => {
            error!("error finding customer");
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
    let sig_address: String = monero::verify_signature(address, data, String::from(&signature)).await;
    if sig_address == utils::ApplicationErrors::LoginError.value() {
        return f_auth;
    }
    let results = customers
        .filter(schema::customers::c_xmr_address.eq(&sig_address))
        .first::<Customer>(connection);
    match results {
        Ok(r) => {
            if r.c_xmr_address != String::from("") {
                let m_access = verify_access(&r.c_xmr_address, &signature).await;
                if !m_access { return Default::default() }
                return f_auth;
            }
            error!("error creating customer");
            Default::default()
        }
        _ => {
            info!("creating new customer");
            create(connection, &sig_address, "", "").await;
            return f_auth;
        }
    }
}

/// Update customer information
pub async fn modify(_id: String, data: String, update_type: i32) -> Customer {
    use self::schema::customers::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    if update_type == UpdateType::Name.value() {
        info!("modify customer name");
        let m = diesel::update(customers.find(_id))
            .set(c_name.eq(data))
            .get_result::<Customer>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        }
    } else if update_type == UpdateType::Pgp.value() {
        info!("modify customer pgp");
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
