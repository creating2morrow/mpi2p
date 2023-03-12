use crate::args;
use crate::customer;
use crate::db;
use crate::models::*;
// use crate::monero;
use crate::utils;
// use crate::vendor;
use clap::Parser;
use log::{debug, info};

use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request};

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;

/// Determine customer or vendor login
pub async fn get_login
(address: String, aid: String, corv: String, cvid: String, signature: String) -> Authorization {
    info!("verify {} login", corv);
    // if corv == utils::LoginType::Customer.value() {
        customer::verify_login(address, aid, cvid, signature).await
    // } // else {
        //vendor::verify_login(address, signature).await
    // }
}

/// Create authorization data to sign and expiration
pub fn create(address: &String) -> Authorization {
    info!("creating auth");
    let aid: String = format!("auth{}", utils::generate_rnd());
    let rnd: String = utils::generate_rnd();
    let created: i64 = chrono::offset::Utc::now().timestamp();
    let token: String = create_token(String::from(address), created);
    let new_auth = Authorization {
        aid,
        created,
        cvid: utils::empty_string(),
        rnd,
        token,
        xmr_address: String::from(address),
    };
    let s = db::Interface::open();
    debug!("insert auth: {:?}", &new_auth);
    let k = &new_auth.aid;
    db::Interface::write(&s.env, &s.handle, k, &Authorization::to_db(&new_auth));
    new_auth
}

/// Authorization lookup for recurring requests
pub fn find(aid: &String) -> Authorization {
    info!("searching for auth: {}", aid);
    let s = db::Interface::open();
    let r = db::Interface::read(&s.env, &s.handle, &String::from(aid));
    debug!("auth read: {}", r);
    if r == utils::empty_string() {
        return Default::default()
    }
    Authorization::from_db(String::from(aid), r)
}

// /// Update new authorization creation time
// async fn update_expiration(_id: &str, address: String) -> Authorization {
//     use self::schema::authorizations::dsl::*;
//     let connection = &mut utils::establish_pgdb_connection().await;
//     info!("modify auth expiration");
//     let time: i64 = chrono::offset::Utc::now().timestamp();
//     // update token as well
//     let token_update = diesel::update(authorizations.find(_id))
//         .set(token.eq(create_token(address, time)))
//         .get_result::<Authorization>(connection);
//     match token_update {
//         Ok(_) => info!("token updated successfully"),
//         Err(_) => error!("error updating token"),
//     }
//     let m = diesel::update(authorizations.find(_id))
//         .set(created.eq(time))
//         .get_result::<Authorization>(connection);
//     match m {
//         Ok(m) => m,
//         Err(_e) => Default::default(),
//     }
// }

// /// Update auth data to sign
// async fn update_data(_id: &str) -> Authorization {
//     use self::schema::authorizations::dsl::*;
//     let connection = &mut utils::establish_pgdb_connection().await;
//     info!("modify auth data");
//     let data: String = utils::generate_rnd();
//     let m = diesel::update(authorizations.find(_id))
//         .set(rnd.eq(data))
//         .get_result::<Authorization>(connection);
//     match m {
//         Ok(m) => m,
//         Err(_e) => Default::default(),
//     }
// }

// /// Called during auth flow to update data to sign and expiration
// pub async fn verify_access(address: &str, signature: &str) -> bool {
//     // look up auth for address
//     let f_auth: Authorization = find(String::from(address)).await;
//     if f_auth.xmr_address != String::from("") {
//         // check expiration, generate new data to sign if necessary
//         let now: i64 = chrono::offset::Utc::now().timestamp();
//         let expiration = get_auth_expiration();
//         if now > f_auth.created + expiration {
//             update_expiration(&f_auth.aid, String::from(f_auth.xmr_address)).await;
//             update_data(&f_auth.aid).await;
//             return false;
//         }
//     }
//     // verify signature on the data if not expired
//     let data = f_auth.rnd;
//     let sig_address: String =
//         monero::verify_signature(String::from(address), data, String::from(signature)).await;
//     if sig_address == utils::ApplicationErrors::LoginError.value() {
//         debug!("signing failed");
//         return false;
//     }
//     info!("auth verified");
//     return true;
// }

/// get the auth expiration command line configuration
fn get_auth_expiration() -> i64 {
    let args = args::Args::parse();
    args.token_timeout * 60
}

fn create_token(address: String, created: i64) -> String {
    let jwt_secret_key = utils::get_jwt_secret_key();
    let key: Hmac<Sha384> = Hmac::new_from_slice(&jwt_secret_key).expect("hash");
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    let expiration = get_auth_expiration() * created;
    claims.insert("address", address);
    claims.insert("expiration", expiration.to_string());
    let token = Token::new(header, claims).sign_with_key(&key);
    String::from(token.expect("expected token").as_str())
}

#[derive(Debug)]
pub struct BearerToken(String);

#[derive(Debug)]
pub enum BearerTokenError {
    Expired,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = BearerTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        let path = request.uri().path().to_string();
        let split1 = path.split("/");
        let mut v1: Vec<String> = split1.map(|s| String::from(s)).collect();
        let address = v1.remove(2);
        debug!("{}", address);
        match token {
            Some(token) => {
                // check validity
                let jwt_secret_key = utils::get_jwt_secret_key();
                let key: Hmac<Sha384> = Hmac::new_from_slice(&jwt_secret_key).expect("");
                let jwt: Result<
                    Token<jwt::Header, BTreeMap<std::string::String, std::string::String>, _>,
                    jwt::Error,
                > = token.verify_with_key(&key);
               return match jwt {
                    Ok(j) => {
                        let claims = j.claims();
                        debug!("claim address: {}", claims["address"]);
                        // verify address
                        if claims["address"] != address {
                            return Outcome::Failure((
                                Status::Unauthorized,
                                BearerTokenError::Invalid,
                            ));
                        }
                        // verify expiration
                        let now: i64 = chrono::offset::Utc::now().timestamp();
                        let expire = match claims["expiration"].parse::<i64>() {
                            Ok(n) => n,
                            Err(_) => 0,
                        };
                        if now > expire {
                            return Outcome::Failure((
                                Status::Unauthorized,
                                BearerTokenError::Expired,
                            ));
                        }
                        Outcome::Success(BearerToken(String::from(token)))
                    }
                    Err(_) => Outcome::Failure((Status::Unauthorized, BearerTokenError::Invalid)),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, BearerTokenError::Missing)),
        }
    }
}
