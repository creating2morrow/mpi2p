use crate::models::*;
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

#[derive(Debug)]
pub enum OrderUpdateType {
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

/// Create a skeleton for order
pub async fn create_new_order(cid: String, pid: String) -> Order {
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
