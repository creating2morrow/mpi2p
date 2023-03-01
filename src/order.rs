use crate::models::*;
use crate::product;
use crate::schema;
use crate::utils;
use diesel::prelude::*;
use log::{debug, info};

// enum StatusType {
//     Delivered,
//     Error,
//     MultisigMissing,
//     MulitsigComplete,
//     Signed,
//     Shipped,
//     Submitted,
// }

// impl StatusType {
//     pub fn value(&self) -> i32 {
//         match *self {
//             StatusType::Delivered => 0,
//             StatusType::Error => 1,
//             StatusType::MultisigMissing => 2,
//             StatusType::MulitsigComplete => 3,
//             StatusType::Signed => 4,
//             StatusType::Shipped => 5,
//             StatusType::Submitted => 6,
//         }
//     }
// }

enum UpdateType {
    CustomerKex1,
    CustomerKex2,
    CustomerKex3,
    CustomerMultisigInfo,
    Deliver,
    Hash,
    Ship,
    VendorKex1,
    VendorKex2,
    VendorKex3,
    VendorMultisigInfo,
    Quantity,
}

impl UpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            UpdateType::CustomerKex1 => 0,
            UpdateType::CustomerKex2 => 1,
            UpdateType::CustomerKex3 => 2,
            UpdateType::CustomerMultisigInfo => 3,
            UpdateType::Deliver => 4,
            UpdateType::Hash => 5,
            UpdateType::Ship => 6,
            UpdateType::VendorKex1 => 7,
            UpdateType::VendorKex2 => 8,
            UpdateType::VendorKex3 => 9,
            UpdateType::VendorMultisigInfo => 10,
            UpdateType::Quantity => 11,
        }
    }
}

/// Create a skeleton for order
pub async fn create(cid: String, pid: String) -> Order {
    use crate::schema::orders;
    let connection = &mut utils::establish_pgdb_connection().await;
    let ts = chrono::offset::Utc::now().timestamp();
    let oid: String = format!("O{}", utils::generate_rnd());
    let new_order = NewOrder {
        orid: &oid,
        c_id: &cid,
        p_id: &pid,
        o_cust_kex_1: "",
        o_cust_kex_2: "",
        o_cust_kex_3: "",
        o_xmr_address: "",
        o_cust_msig_info: "",
        o_date: &ts,
        o_deliver_date: &0,
        o_ship_date: &0,
        o_hash: "",
        o_msig_prepare: "",
        o_msig_make: "",
        o_msig_kex_1: "",
        o_msig_kex_2: "",
        o_msig_kex_3: "",
        o_status: "",
        o_quantity: &0,
        o_vend_kex_1: "",
        o_vend_kex_2: "",
        o_vend_kex_3: "",
        o_vend_msig_info: "",
    };
    debug!("insert order: {:?}", new_order);
    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(connection)
        .expect("error saving new order")
}


/// Modify order lifecycle
pub async fn modify(_id: String, pid: String, data: String, update_type: i32) -> Order {
    use self::schema::orders::dsl::*;
    let t_id: String = String::from(&_id);
    let is_customer = is_customer(t_id);
    let connection = &mut utils::establish_pgdb_connection().await;
    // this else if chain is awful, TODO: refactor
    if update_type == UpdateType::CustomerKex1.value() && is_customer {
        info!("modify order customer kex 1");
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_1.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerKex2.value() && is_customer {
        info!("modify customer kex 2");
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_2.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerKex3.value() && is_customer {
        info!("modify customer kex 3");
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_3.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerMultisigInfo.value() && is_customer {
        info!("modify customer multisig info");
        let m = diesel::update(orders.find(_id))
            .set(o_cust_msig_info.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Deliver.value() && !is_customer {
        info!("modify devliver date");
        let deliver_date = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(orders.find(_id))
            .set(o_deliver_date.eq(deliver_date))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Hash.value() && is_customer {
        info!("modify order hash");
        let m = diesel::update(orders.find(_id))
            .set(o_hash.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Ship.value() && !is_customer {
        info!("modify order ship date");
        let ship_date = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(orders.find(_id))
            .set(o_ship_date.eq(ship_date))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex1.value() && !is_customer {
        info!("modify order customer kex 1");
        let m = diesel::update(orders.find(_id))
            .set(o_vend_kex_1.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex2.value() {
        info!("modify vendor kex 2");
        let m = diesel::update(orders.find(_id))
            .set(o_vend_kex_2.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex3.value() && !is_customer {
        info!("modify vendor kex 3");
        let m = diesel::update(orders.find(_id))
            .set(o_vend_kex_3.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorMultisigInfo.value() && !is_customer {
        info!("modify vendor multisig info");
        let m = diesel::update(orders.find(_id))
            .set(o_vend_msig_info.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Quantity.value() && is_customer {
        info!("modify quantity");
        // check qty/in_stock
        let m_product: Product = product::find(pid).await;
        let qty = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let is_invalid_qty: bool = !m_product.in_stock || m_product.qty == 0 || qty > m_product.qty;
        if is_invalid_qty {
            return Default::default();
        }
        let m = diesel::update(orders.find(_id))
            .set(o_quantity.eq(qty))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    }
    Default::default()
}

pub fn is_customer(id: String) -> bool {
    let first: char = id.chars().nth(0).unwrap();
    debug!("id starts with: {}", first);
    return first == 'C'
}
