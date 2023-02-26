use crate::models::*;
use crate::logger;
use crate::schema;
use crate::utils;
use diesel::prelude::*;

#[derive(Debug)]
pub enum OrderStatusType {
    Delivered,
    Error,
    MultisigMissing,
    MulitsigComplete,
    Signed,
    Shipped,
    Submitted,
}

impl OrderStatusType {
    pub fn value(&self) -> i32 {
        match *self {
            OrderStatusType::Delivered => 0,
            OrderStatusType::Error => 1,
            OrderStatusType::MultisigMissing => 2,
            OrderStatusType::MulitsigComplete => 3,
            OrderStatusType::Signed => 4,
            OrderStatusType::Shipped => 5,
            OrderStatusType::Submitted => 6,
        }
    }
}

enum OrderUpdateType {
    CustomerKex1,
    CustomerKex2,
    CustomerKex3,
    CustomerMultisigInfo,
    Deliver,
    Hash,
    SignMultisig,
    Ship,
    SubmitMultisig,
    Status,
    VendorKex1,
    VendorKex2,
    VendorKex3,
    VendorMultisigInfo,
    Quantity,
}

impl OrderUpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            OrderUpdateType::CustomerKex1 => 0,
            OrderUpdateType::CustomerKex2 => 1,
            OrderUpdateType::CustomerKex3 => 2,
            OrderUpdateType::CustomerMultisigInfo => 3,
            OrderUpdateType::Deliver => 3,
            OrderUpdateType::Hash => 3,
            OrderUpdateType::SignMultisig => 3,
            OrderUpdateType::Ship => 3,
            OrderUpdateType::SubmitMultisig => 3,
            OrderUpdateType::Status => 3,
            OrderUpdateType::VendorKex1 => 3,
            OrderUpdateType::VendorKex2 => 3,
            OrderUpdateType::VendorKex3 => 3,
            OrderUpdateType::VendorMultisigInfo => 3,
            OrderUpdateType::Quantity => 3,
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
    };
    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(connection)
        .expect("Error saving new order")
}

pub async fn modify(_id: String, data: String, update_type: i32) -> Product {
    use self::schema::orders::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    
    if update_type == OrderUpdateType::CustomerKex1.value() {
        logger::log(logger::LogLevel::INFO, "Modify order customer kex 1.").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_1.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == OrderUpdateType::CustomerKex2.value() {
        logger::log(logger::LogLevel::INFO, "Modify customer kex 2.").await;
        let m = diesel::update(orders.find(_id))
            .set(o_cust_kex_2.eq(data))
            .get_result::<Order>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == OrderUpdateType::CustomerKex3.value() {
        logger::log(logger::LogLevel::INFO, "Modify customer kex 3.").await;
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