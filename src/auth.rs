use crate::args;
use crate::customer;
use crate::logger;
use crate::models::*;
use crate::monero;
use crate::schema;
use crate::utils;
use crate::vendor;
use clap::Parser;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub async fn get_login_auth(address: String, corv: String, signature: String) -> Authorization {
    if corv == utils::LoginType::Customer.value() {
        customer::verify_customer_login(address, signature).await
    } else {
        vendor::verify_vendor_login(address, signature).await
    }
}

pub async fn create_auth(conn: &mut PgConnection, address: String) -> Authorization {
    use crate::schema::authorizations;
    let aid: String = utils::generate_rnd();
    let rnd: String = utils::generate_rnd();
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

pub async fn find_auth(address: String) -> Authorization {
    use self::schema::authorizations::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let results = authorizations
        .filter(schema::authorizations::xmr_address.eq(address))
        .load::<Authorization>(connection);
    match results {
        Ok(mut r) => {
            if &r.len() > &0 {
                logger::log(logger::LogLevel::INFO, "Found auth.").await;
                r.remove(0)
            } else {
                Default::default()
            }
        }
        _ => {
            logger::log(logger::LogLevel::ERROR, "Error finding auth.").await;
            Default::default()
        }
    }
}

async fn update_auth_expiration(_id: &str) -> Authorization {
    use self::schema::authorizations::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    logger::log(logger::LogLevel::INFO, "Modify auth expiration.").await;
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
    let connection = &mut utils::establish_pgdb_connection().await;
    logger::log(logger::LogLevel::INFO, "Modify auth data.").await;
    let data: String = utils::generate_rnd();
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
        monero::verify_signature(String::from(address), data, String::from(signature)).await;
    if sig_address == utils::ApplicationErrors::LoginError.value() {
        return false;
    }
    return true;
}

fn get_auth_expiration() -> i64 {
    let args = args::Args::parse();
    args.token_timeout * 60
}
