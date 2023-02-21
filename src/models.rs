use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Customer {
    pub cid: String,
    pub c_xmr_address: String,
    pub c_name: String,
    pub c_pgp: String,
}

impl Default for Customer {
    fn default() -> Self {
        Customer {
            cid: String::from(""),
            c_xmr_address: String::from(""),
            c_name: String::from(""),
            c_pgp: String::from(""),
        }
    }
}

#[derive(Queryable, Debug)]
pub struct Vendor {
    pub vid: String,
    pub v_xmr_address: String,
    pub v_name: String,
    pub v_description: String,
    pub v_pgp: String,
    pub active: bool,
}

impl Default for Vendor {
    fn default() -> Self {
        Vendor {
            vid: String::from(""),
            v_xmr_address: String::from(""),
            v_name: String::from(""),
            v_description: String::from(""),
            v_pgp: String::from(""),
            active: false,
        }
    }
}

#[derive(Queryable, Debug)]
pub struct Product {
    pub pid: String,
    pub v_id: String,
    pub in_stock: bool,
    pub p_description: String,
    pub p_name: String,
    pub p_price: i64,
    pub qty: i64,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            pid: String::from(""),
            v_id: String::from(""),
            in_stock: false,
            p_description: String::from(""),
            p_name: String::from(""),
            p_price: 0,
            qty: 0,
        }
    }
}

// #[derive(Queryable)]
// pub struct Order {
//     pub orid: i32,
//     pub c_id: String,
//     pub p_id: String,
//     pub o_date: i32,
//     pub o_hash: String,
// }

#[derive(Queryable)]
pub struct Authorization {
    pub aid: String,
    pub created: i64,
    pub rnd: String,
    pub xmr_address: String,
}

impl Default for Authorization {
    fn default() -> Self {
        Authorization {
            aid: String::from(""),
            created: 0,
            rnd: String::from(""),
            xmr_address: String::from(""),
        }
    }
}

use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::customers)]
pub struct NewCustomer<'a> {
    pub cid: &'a str,
    pub c_xmr_address: &'a str,
    pub c_name: &'a str,
    pub c_pgp: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = schema::vendors)]
pub struct NewVendor<'a> {
    pub vid: &'a str,
    pub v_xmr_address: &'a str,
    pub v_name: &'a str,
    pub v_description: &'a str,
    pub v_pgp: &'a str,
    pub active: &'a bool,
}

#[derive(Insertable)]
#[diesel(table_name = schema::products)]
pub struct NewProduct<'a> {
    pub pid: &'a str,
    pub v_id: &'a str,
    pub in_stock: &'a bool,
    pub p_description: &'a str,
    pub p_name: &'a str,
    pub p_price: &'a i64,
    pub qty: &'a i64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::authorizations)]
pub struct NewAuthorization<'a> {
    pub aid: &'a str,
    pub created: &'a i64,
    pub rnd: &'a str,
    pub xmr_address: &'a str,
}
