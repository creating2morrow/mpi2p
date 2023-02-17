use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Customer {
    pub id: i32,
    pub c_xmr_address: String,
    pub c_name: String,
    pub c_pgp: String,
}

// #[derive(Queryable)]
// pub struct Order {
//     pub id: i32,
//     pub c_id: String,
//     pub p_id: String,
//     pub o_date: i32,
//     pub o_hash: String,
// }

#[derive(Queryable, Debug)]
pub struct Vendor {
    pub id: i32,
    pub v_xmr_address: String,
    pub v_name: String,
    pub v_description: String,
    pub v_pgp: String,
    pub active: bool,
}

#[derive(Queryable, Debug)]
pub struct Product {
    pub id: i32,
    pub v_id: i32,
    pub in_stock: bool,
    pub p_description: String,
    pub p_name: String,
    pub p_price: i32,
    pub qty: i32,
}

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::customers)]
pub struct NewCustomer<'a> {
    pub c_xmr_address: &'a str,
    pub c_name: &'a str,
    pub c_pgp: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = schema::vendors)]
pub struct NewVendor<'a> {
    pub v_xmr_address: &'a str,
    pub v_name: &'a str,
    pub v_description: &'a str,
    pub v_pgp: &'a str,
    pub active: &'a bool,
}

#[derive(Insertable)]
#[diesel(table_name = schema::products)]
pub struct NewProduct<'a> {
    pub v_id: &'a i32,
    pub in_stock: &'a bool,
    pub p_description: &'a str,
    pub p_name: &'a str,
    pub p_price: &'a i32,
    pub qty: &'a i32,
}
