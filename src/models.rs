use diesel::prelude::*;

/*
  Making changes here involves 4 possible locations:
    1: migrations/<model_dir>/UP.sql
    2: models.rs Queryable and Insertable structs
    3: reqres.rs request, response structs
    4: reqres.rs response::build()
*/

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

#[derive(Queryable, Debug)]
pub struct Order {
    pub orid: String,
    pub c_id: String,
    pub p_id: String,
    pub v_id: String,
    pub o_xmr_address: String,
    pub o_cust_msig_info: String,
    pub o_cust_msig_txset: String,
    pub o_cust_kex_1: String,
    pub o_cust_kex_2: String,
    pub o_cust_kex_3: String,
    pub o_date: i64,
    pub o_deliver_date: i64,
    pub o_ship_date: i64,
    pub o_hash: String,
    pub o_msig_prepare: String,
    pub o_msig_make: String,
    pub o_msig_kex_1: String,
    pub o_msig_kex_2: String,
    pub o_msig_kex_3: String,
    pub o_status: String,
    pub o_quantity: i64,
    pub o_vend_kex_1: String,
    pub o_vend_kex_2: String,
    pub o_vend_kex_3: String,
    pub o_vend_msig_info: String,
    pub o_vend_msig_txset: String
}

impl Default for Order {
    fn default() -> Self {
        Order {
            orid: String::from(""),
            c_id: String::from(""),
            p_id: String::from(""),
            v_id: String::from(""),
            o_xmr_address: String::from(""),
            o_cust_msig_info: String::from(""),
            o_cust_msig_txset: String::from(""),
            o_cust_kex_1: String::from(""),
            o_cust_kex_2: String::from(""),
            o_cust_kex_3: String::from(""),
            o_date: 0,
            o_deliver_date: 0,
            o_ship_date: 0,
            o_hash: String::from(""),
            o_msig_prepare: String::from(""),
            o_msig_make: String::from(""),
            o_msig_kex_1: String::from(""),
            o_msig_kex_2: String::from(""),
            o_msig_kex_3: String::from(""),
            o_status: String::from(""),
            o_quantity: 0,
            o_vend_kex_1: String::from(""),
            o_vend_kex_2: String::from(""),
            o_vend_kex_3: String::from(""),
            o_vend_msig_info: String::from(""),
            o_vend_msig_txset: String::from(""),
        }
    }
}

#[derive(Queryable, Debug)]
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

#[derive(Queryable, Debug)]
pub struct Dispute {
    pub did: String,
    pub created: i64,
    pub orid: String,
    pub tx_set: String,
}

impl Default for Dispute {
    fn default() -> Self {
        Dispute {
            did: String::from(""),
            created: 0,
            orid: String::from(""),
            tx_set: String::from(""),
        }
    }
}

use crate::schema;

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::customers)]
pub struct NewCustomer<'a> {
    pub cid: &'a str,
    pub c_xmr_address: &'a str,
    pub c_name: &'a str,
    pub c_pgp: &'a str,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::vendors)]
pub struct NewVendor<'a> {
    pub vid: &'a str,
    pub v_xmr_address: &'a str,
    pub v_name: &'a str,
    pub v_description: &'a str,
    pub v_pgp: &'a str,
    pub active: &'a bool,
}

#[derive(Insertable, Debug)]
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

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::authorizations)]
pub struct NewAuthorization<'a> {
    pub aid: &'a str,
    pub created: &'a i64,
    pub rnd: &'a str,
    pub xmr_address: &'a str,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::disputes)]
pub struct NewDispute<'a> {
    pub did: &'a str,
    pub created: &'a i64,
    pub orid: &'a str,
    pub tx_set: &'a str,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::orders)]
pub struct NewOrder<'a> {
    pub orid: &'a str,
    pub c_id: &'a str,
    pub p_id: &'a str,
    pub v_id: &'a str,
    pub o_xmr_address: &'a str,
    pub o_cust_msig_txset: &'a str,
    pub o_date: &'a i64,
    pub o_deliver_date: &'a i64,
    pub o_ship_date: &'a i64,
    pub o_hash: &'a str,
    pub o_cust_msig_info: &'a str,
    pub o_cust_kex_1: &'a str,
    pub o_cust_kex_2: &'a str,
    pub o_cust_kex_3: &'a str,
    pub o_msig_prepare: &'a str,
    pub o_msig_make: &'a str,
    pub o_msig_kex_1: &'a str,
    pub o_msig_kex_2: &'a str,
    pub o_msig_kex_3: &'a str,
    pub o_status: &'a str,
    pub o_quantity: &'a i64,
    pub o_vend_kex_1: &'a str,
    pub o_vend_kex_2: &'a str,
    pub o_vend_kex_3: &'a str,
    pub o_vend_msig_info: &'a str,
    pub o_vend_msig_txset: &'a str,
}
