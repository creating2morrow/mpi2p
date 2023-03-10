use hex;
use rand_core::RngCore;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use clap::Parser;
use crate::args;

#[derive(Debug, PartialEq)]
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
pub enum ApplicationErrors {
    LoginError,
    UnknownError,
}

impl ApplicationErrors {
    pub fn value(&self) -> String {
        match *self {
            ApplicationErrors::LoginError => String::from("LoginError"),
            ApplicationErrors::UnknownError => String::from("UnknownError"),
        }
    }
}

/// Helper for connecting to db on ORM
pub async fn establish_pgdb_connection() -> PgConnection {
    let args = args::Args::parse();
    let db_string: String = String::from(args.postgres_db_url);
    PgConnection::establish(&db_string)
        .unwrap_or_else(|_| panic!("error connecting to {}", db_string))
}

/// Random data generation for authorization signing
pub fn generate_rnd() -> String {
    let mut data = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut data);
    hex::encode(data)
}

/// Helper for separation of dev and prod concerns
pub fn get_release_env() -> ReleaseEnvironment {
    let args = args::Args::parse();
    let env = String::from(args.release_env);
    if env == "prod" {
        return ReleaseEnvironment::Production;
    } else {
        return ReleaseEnvironment::Development;
    }
}

/// Helper for separation of dev and prod concerns
pub fn get_jwt_secret_key() -> Vec<u8> {
    let args = args::Args::parse();
    let key = String::from(args.jwt_secret_key);
    key.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_rnd_test() {
        let rnd = generate_rnd();
        let valid_length = 64;
        assert_eq!(rnd.len(), valid_length);
    }

    #[test]
    fn args_test() {
        let env = get_release_env();
        assert_eq!(env, ReleaseEnvironment::Development)
    }

}
