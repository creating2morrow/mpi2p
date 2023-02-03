pub mod models;
pub mod schema;

use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/*
 TODO:
   1)  get_customer
   2)  update_customer
   3)  create_product
   4)  get_product
   5)  update_product
   6)  create_vendor
   7)  get_vendor
   8)  update_vendor
   9)  create_order
   10) get_order
   11) update_order
*/

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_customer(conn: &mut PgConnection, c_name: &str, c_pgp: &str) -> Customer {
    use crate::schema::customers;

    let new_customer = NewCustomer { c_name, c_pgp };

    diesel::insert_into(customers::table)
        .values(&new_customer)
        .get_result(conn)
        .expect("Error saving new customer")
}
