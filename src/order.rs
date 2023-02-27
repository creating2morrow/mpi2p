use crate::models::*;
use crate::logger;
use crate::schema;
use crate::utils;
use diesel::prelude::*;

#[derive(Debug)]
pub enum StatusType {
    Delivered,
    Error,
    MultisigMissing,
    MulitsigComplete,
    Signed,
    Shipped,
    Submitted,
}

impl StatusType {
    pub fn value(&self) -> i32 {
        match *self {
            StatusType::Delivered => 0,
            StatusType::Error => 1,
            StatusType::MultisigMissing => 2,
            StatusType::MulitsigComplete => 3,
            StatusType::Signed => 4,
            StatusType::Shipped => 5,
            StatusType::Submitted => 6,
        }
    }
}

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
    let oid: String = utils::generate_rnd();
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
    logger::log(logger::LogLevel::DEBUG,
        &format!("insert order: {:?}", new_order)).await;
    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(connection)
        .expect("error saving new order")
}


/// TODO: modification auth needs to be checked per update type
pub async fn modify(_id: String, data: String, update_type: i32) -> Product {
    use self::schema::orders::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    // this else if chain is awful, TODO: refactor
    if update_type == UpdateType::CustomerKex1.value() {
        logger::log(logger::LogLevel::INFO, "modify order customer kex 1").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_1.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerKex2.value() {
        logger::log(logger::LogLevel::INFO, "modify customer kex 2").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_2.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerKex3.value() {
        logger::log(logger::LogLevel::INFO, "modify customer kex 3").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_3.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerMultisigInfo.value() {
        logger::log(logger::LogLevel::INFO, "modify customer multisig info").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_msig_info.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Deliver.value() {
        logger::log(logger::LogLevel::INFO, "modify devliver date").await;
        let deliver_date = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(orders.find(_id))
            .set(o_deliver_date.eq(deliver_date))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Hash.value() {
        logger::log(logger::LogLevel::INFO, "modify order hash").await;
        let m = diesel::update(orders.find(_id))
            .set(o_hash.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Ship.value() {
        logger::log(logger::LogLevel::INFO, "modify order ship date").await;
        let ship_date = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(orders.find(_id))
            .set(o_ship_date.eq(ship_date))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerKex3.value() {
        logger::log(logger::LogLevel::INFO, "modify customer kex 3").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_3.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex1.value() {
        logger::log(logger::LogLevel::INFO, "modify order customer kex 1").await;
        let m = diesel::update(orders.find(_id))
            .set(o_vend_kex_1.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex2.value() {
        logger::log(logger::LogLevel::INFO, "modify vendor kex 2").await;
        let m = diesel::update(orders.find(_id))
            .set(o_vend_kex_2.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex3.value() {
        logger::log(logger::LogLevel::INFO, "modify vendor kex 3").await;
        let m = diesel::update(orders.find(_id))
            .set(o_vend_kex_3.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorMultisigInfo.value() {
        logger::log(logger::LogLevel::INFO, "modify vendor multisig info").await;
        let m = diesel::update(orders.find(_id))
            .set(o_vend_msig_info.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Quantity.value() {
        logger::log(logger::LogLevel::INFO, "modify vendor kex 3").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_3.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    }
    Default::default()
}
