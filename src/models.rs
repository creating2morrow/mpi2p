use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Customer {
    pub id: i32,
    pub c_xmr_address: String,
    pub c_name: String,
    pub c_pgp: String,
}

#[derive(Queryable)]
pub struct Order {
    pub id: i32,
    pub c_id: String,
    pub p_id: String,
    pub o_date: i32,
    pub o_hash: String,
}

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::customers)]
pub struct NewCustomer<'a> {
    pub c_xmr_address: &'a str,
    pub c_name: &'a str,
    pub c_pgp: &'a str,
}
