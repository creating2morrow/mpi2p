use crate::utils;

/*
  Making changes here involves 2 possible locations:
    1: reqres.rs request, response structs
    2: reqres.rs response::build()
*/

#[derive(Debug)]
pub struct Customer {
    pub cid: String,
    pub xmr_address: String,
    pub name: String,
    pub pgp: String,
}

impl Default for Customer {
    fn default() -> Self {
        Customer {
            cid: utils::empty_string(),
            xmr_address: utils::empty_string(),
            name: utils::empty_string(),
            pgp: utils::empty_string(),
        }
    }
}

impl Customer {
    pub fn to_db(c: &Customer) -> String {
        format!("{}:{}:{}",
            c.name, c.pgp, c.xmr_address)
    }
    pub fn from_db(k: String, v: String) -> Customer {
        let values = v.split(":");
        let mut v: Vec<String> = values.map(|s| String::from(s)).collect();
        let  name = v.remove(0);
        let pgp = v.remove(0);
        let xmr_address = v.remove(0);
        Customer { cid: k, name, pgp, xmr_address }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub o_subaddress: String,
    pub o_status: String,
    pub o_quantity: i64,
    pub o_vend_kex_1: String,
    pub o_vend_kex_2: String,
    pub o_vend_kex_3: String,
    pub o_vend_msig_info: String,
    pub o_vend_msig_txset: String,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            orid: utils::empty_string(),
            c_id: utils::empty_string(),
            p_id: utils::empty_string(),
            v_id: utils::empty_string(),
            o_xmr_address: utils::empty_string(),
            o_cust_msig_info: utils::empty_string(),
            o_cust_msig_txset: utils::empty_string(),
            o_cust_kex_1: utils::empty_string(),
            o_cust_kex_2: utils::empty_string(),
            o_cust_kex_3: utils::empty_string(),
            o_date: 0,
            o_deliver_date: 0,
            o_ship_date: 0,
            o_hash: utils::empty_string(),
            o_msig_prepare: utils::empty_string(),
            o_msig_make: utils::empty_string(),
            o_msig_kex_1: utils::empty_string(),
            o_msig_kex_2: utils::empty_string(),
            o_msig_kex_3: utils::empty_string(),
            o_subaddress: utils::empty_string(),
            o_status: utils::empty_string(),
            o_quantity: 0,
            o_vend_kex_1: utils::empty_string(),
            o_vend_kex_2: utils::empty_string(),
            o_vend_kex_3: utils::empty_string(),
            o_vend_msig_info: utils::empty_string(),
            o_vend_msig_txset: utils::empty_string(),
        }
    }
}

#[derive(Debug)]
pub struct Authorization {
    pub aid: String,
    pub created: i64,
    pub cvid: String,
    pub rnd: String,
    pub token: String,
    pub xmr_address: String,
}

impl Default for Authorization {
    fn default() -> Self {
        Authorization {
            aid: utils::empty_string(),
            created: 0,
            cvid: utils::empty_string(),
            rnd: utils::empty_string(),
            token: utils::empty_string(),
            xmr_address: utils::empty_string(),
        }
    }
}

impl Authorization {
    pub fn to_db(a: &Authorization) -> String {
        format!("{}:{}:{}:{}:{}",
            a.created, a.cvid, a.rnd, a.token, a.xmr_address)
    }
    pub fn from_db(k: String, v: String) -> Authorization {
        let values = v.split(":");
        let mut v: Vec<String> = values.map(|s| String::from(s)).collect();
        let created_str = v.remove(0);
        let created = match created_str.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let cvid = v.remove(0);
        let rnd = v.remove(0);
        let token = v.remove(0);
        let xmr_address = v.remove(0);
        Authorization { aid: k, created, cvid, rnd, token, xmr_address }
    }
    pub fn update_cvid(a: Authorization, cvid: String) -> Authorization {
        Authorization { 
            aid: a.aid, 
            created: a.created, 
            cvid, 
            rnd: a.rnd, 
            token: a.token,
            xmr_address: a.xmr_address
        }
    }
    pub fn update_expiration(a: Authorization, created: i64, rnd: String, token: String) -> Authorization {
        Authorization { 
            aid: a.aid, 
            created, 
            cvid: a.cvid, 
            rnd, 
            token,
            xmr_address: a.xmr_address
        }
    }
}

#[derive(Debug)]
pub struct Dispute {
    pub did: String,
    pub created: i64,
    pub orid: String,
    pub tx_set: String,
}

impl Default for Dispute {
    fn default() -> Self {
        Dispute {
            did: utils::empty_string(),
            created: 0,
            orid: utils::empty_string(),
            tx_set: utils::empty_string(),
        }
    }
}
