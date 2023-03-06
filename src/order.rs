use crate::customer;
use crate::models::*;
use crate::monero;
use crate::product;
use crate::reqres;
use crate::schema;
use crate::utils;
use crate::vendor;
use diesel::prelude::*;
use log::{debug, error, info};

// TODO: dispute handling logic

enum StatusType {
    Delivered,
    // Dispute,
    // Error,
    MultisigMissing,
    MulitsigComplete,
    // Signed,
    Shipped,
    // Submitted,
}

impl StatusType {
    pub fn value(&self) -> String {
        match *self {
            StatusType::Delivered => String::from("Delivered"),
            // StatusType::Dispute => String::from("Dispute"),
            // StatusType::Error => String::from("Error"),
            StatusType::MultisigMissing => String::from("MultisigMissing"),
            StatusType::MulitsigComplete => String::from("MulitsigComplete"),
            // StatusType::Signed => String::from("Signed"),
            StatusType::Shipped => String::from("Shipped"),
            // StatusType::Submitted => String::from("Submitted"),
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
            UpdateType::CustomerKex1 => 0,         // make output from customer
            UpdateType::CustomerKex2 => 1,         // use this for funding kex
            UpdateType::CustomerKex3 => 2,         // might need this later?
            UpdateType::CustomerMultisigInfo => 3, // prepare output from customer
            UpdateType::Deliver => 4,              // customer has received the item, released funds
            UpdateType::Hash => 5,                 // tx hash from funding the wallet order
            UpdateType::Ship => 6,                 // update ship date, app doesn't store tracking numbers
            UpdateType::VendorKex1 => 7,           // make output from vendor
            UpdateType::VendorKex2 => 8,           // use this for funding kex
            UpdateType::VendorKex3 => 9,           // might need this later?
            UpdateType::VendorMultisigInfo => 10,  // prepare output from vendor
            UpdateType::Quantity => 11,            // this can be updated until wallet is funded
        }
    }
}

/// Create a skeleton for order
pub async fn create(cid: String, pid: String) -> Order {
    use crate::schema::orders;
    let connection = &mut utils::establish_pgdb_connection().await;
    let ts = chrono::offset::Utc::now().timestamp();
    let oid: String = format!("O{}", utils::generate_rnd());
    let m_product: Product = product::find(String::from(&pid)).await;
    let new_order = NewOrder {
        orid: &oid,
        c_id: &cid,
        p_id: &pid,
        v_id: &m_product.v_id,
        o_cust_kex_1: "",
        o_cust_kex_2: "",
        o_cust_kex_3: "",
        o_xmr_address: "",
        o_cust_msig_info: "",
        o_cust_msig_txset: "",
        o_date: &ts,
        o_deliver_date: &0,
        o_ship_date: &0,
        o_hash: "",
        o_msig_prepare: "",
        o_msig_make: "",
        o_msig_kex_1: "",
        o_msig_kex_2: "",
        o_msig_kex_3: "",
        o_status: &StatusType::MultisigMissing.value(),
        o_quantity: &0,
        o_vend_kex_1: "",
        o_vend_kex_2: "",
        o_vend_kex_3: "",
        o_vend_msig_info: "",
        o_vend_msig_txset: "",
    };
    debug!("insert order: {:?}", new_order);
    let m_wallet = monero::create_wallet(String::from(&oid)).await;
    if !m_wallet {
        error!("error creating wallet");
    }
    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(connection)
        .expect("error saving new order")
    // create wallet for the order
}

/// Modify order lifecycle
pub async fn modify(_id: String, pid: String, data: String, update_type: i32) -> Order {
    use self::schema::orders::dsl::*;
    let t_id: String = String::from(&_id);
    let is_customer = is_customer(t_id);
    let connection = &mut utils::establish_pgdb_connection().await;
    let old_search = orders
        .filter(schema::orders::orid.eq(String::from(&_id)))
        .first::<Order>(connection);
    let old = match old_search {
        Ok(r) => r,
        _ => {
            error!("error finding old order");
            Default::default()
        }
    };
    // this else if chain is awful, TODO: refactor
    if update_type == UpdateType::CustomerKex1.value() && is_customer {
        info!("modify order customer kex 1");
        let m = diesel::update(orders.find(String::from(&_id)))
            .set(o_cust_kex_1.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => {
                if old.o_vend_kex_1 != String::from("") {
                    finalize(connection, String::from(&_id), old).await;
                }
                m
            },
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::CustomerKex2.value() && is_customer {
        info!("modify customer kex 2");
        let m = diesel::update(orders.find(String::from(&_id)))
            .set(o_cust_kex_2.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => {
                if old.o_vend_kex_2 != String::from("") {
                    update_multisig_info(connection, String::from(&_id), old).await;
                }
                m
            },
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
        let m = diesel::update(orders.find(String::from(&_id)))
            .set(o_cust_msig_info.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => {
                if old.o_vend_msig_info != String::from("") {
                    prepare_and_make(connection, String::from(&_id), old).await;
                }
                m
            }
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Deliver.value() && !is_customer {
        info!("modify devliver date");
        let deliver_date = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(orders.find(_id))
            .set((o_deliver_date.eq(deliver_date), o_status.eq(StatusType::Delivered.value())))
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
            .set((o_ship_date.eq(ship_date), o_status.eq(StatusType::Shipped.value())))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex1.value() && !is_customer {
        info!("modify order customer kex 1");
        let m = diesel::update(orders.find(String::from(&_id)))
            .set(o_vend_kex_1.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => {
                if old.o_cust_kex_1 != String::from("") {
                    finalize(connection, String::from(&_id), old).await;
                }
                m
            }
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::VendorKex2.value() && !is_customer {
        info!("modify vendor kex 2");
        let m = diesel::update(orders.find(String::from(&_id)))
            .set(o_vend_kex_2.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => {
                if old.o_cust_kex_1 != String::from("") {
                    update_multisig_info(connection, String::from(&_id), old).await;
                }
                m
            },
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
        let m = diesel::update(orders.find(String::from(&_id)))
            .set(o_vend_msig_info.eq(data))
            .get_result::<Order>(connection);
        return match m {
            Ok(m) => {
                if old.o_cust_msig_info != String::from("") {
                    prepare_and_make(connection, String::from(&_id), old).await;
                }
                m
            },
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

/// Lookup order
pub async fn find(oid: String) -> Order {
    use self::schema::orders::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let result = orders
        .filter(schema::orders::orid.eq(oid))
        .first::<Order>(connection);
    match result {
        Ok(r) => r,
        _ => {
            error!("error finding order");
            Default::default()
        }
    }
}

/// Lookup all orders for customer or vendor
pub async fn find_all(address: String, corv: String) -> Vec<Order> {
    use self::schema::orders::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    if corv == utils::LoginType::Customer.value() {
        let m_cust: Customer = customer::find(address).await;
        let results = orders
            .filter(schema::orders::c_id.eq(m_cust.cid))
            .load::<Order>(connection);
        return match results {
            Ok(r) => {
                info!("found customer orders");
                r
            }
            _ => {
                error!("error finding customer orders");
                let v: Vec<Order> = Vec::new();
                v
            }
        };
    } else {
        let m_vend: Vendor = vendor::find(address).await;
        let results = orders
            .filter(schema::orders::v_id.eq(m_vend.vid))
            .load::<Order>(connection);
        return match results {
            Ok(r) => {
                info!("found vendor orders");
                r
            }
            _ => {
                error!("error finding vendor orders");
                let v: Vec<Order> = Vec::new();
                v
            }
        };
    };
}

pub fn is_customer(id: String) -> bool {
    let first: char = id.chars().nth(0).unwrap();
    debug!("id starts with: {}", first);
    return first == 'C';
}

/// Attempt to update prepare and make multisig info for the app
async fn prepare_and_make(connection: &mut PgConnection, _id: String, old: Order) {
    let mut m_wallet = monero::open_wallet(String::from(&old.orid)).await;
    if !m_wallet {
        error!("error opening wallet {}", &old.orid);
    }
    use self::schema::orders::dsl::*;
    let mut info: Vec<String> = Vec::new();
    info.push(old.o_vend_msig_info);
    info.push(old.o_cust_msig_info);
    let app_prepare: reqres::XmrRpcPrepareResponse = monero::prepare_wallet().await;
    let prepare_update = diesel::update(orders.find(String::from(&_id)))
        .set(o_msig_prepare.eq(String::from(&app_prepare.result.multisig_info)))
        .get_result::<Order>(connection);
    match prepare_update {
        Ok(_) => info!("prepare info update"),
        Err(_) => error!("error updating prepare info"),
    };
    let make: reqres::XmrRpcMakeResponse = monero::make_wallet(info).await;
    let make_update = diesel::update(orders.find(_id))
        .set(o_msig_kex_1.eq(make.result.multisig_info))
        .get_result::<Order>(connection);
    match make_update {
        Ok(_) => info!("make info update"),
        Err(_) => error!("error updating make info"),
    };
    m_wallet = monero::close_wallet(String::from(&old.orid)).await;
    if !m_wallet {
        error!("error closing wallet {}", &old.orid);
    }
}

/// Attempts to finalize the wallet
async fn finalize(connection: &mut PgConnection, _id: String, old: Order) {
    let mut m_wallet = monero::open_wallet(String::from(&old.orid)).await;
    if !m_wallet {
        error!("error opening wallet {}", &old.orid);
    }
    use self::schema::orders::dsl::*;
    let mut info: Vec<String> = Vec::new();
    info.push(old.o_vend_kex_1);
    info.push(old.o_cust_kex_1);
    let app_finalize: reqres::XmrRpcFinalizeResponse = monero::finalize_wallet(info).await;
    let finalize_update = diesel::update(orders.find(String::from(&_id)))
        .set((o_xmr_address.eq(String::from(&app_finalize.result.address)),
        o_status.eq(StatusType::MulitsigComplete.value())
        ))
        .get_result::<Order>(connection);
    match finalize_update {
        Ok(_) => info!("finalize info update"),
        Err(_) => error!("error finalizing info"),
    };
    m_wallet = monero::close_wallet(String::from(&old.orid)).await;
    if !m_wallet {
        error!("error closing wallet {}", &old.orid);
    }
}

/// Used to update key images after funding the multisig wallet
async fn update_multisig_info(connection: &mut PgConnection, _id: String, old: Order) {
    let mut m_wallet = monero::open_wallet(String::from(&old.orid)).await;
    if !m_wallet {
        error!("error opening wallet {}", &old.orid);
    }
    use self::schema::orders::dsl::*;
    let mut info: Vec<String> = Vec::new();
    info.push(old.o_vend_kex_2);
    info.push(old.o_cust_kex_2);
    let import: reqres::XmrRpcImportResponse = monero::import_multisig_info(info).await;
    if import.result.n_outputs == 0 {
        error!("error importing multisig info");
    }
    let export: reqres::XmrRpcExportResponse = monero::export_multisig_info().await;
    let info_update = diesel::update(orders.find(String::from(&_id)))
        .set(o_msig_kex_2.eq(String::from(&export.result.info)))
        .get_result::<Order>(connection);
    match info_update {
        Ok(_) => info!("msig info update"),
        Err(_) => error!("error updating msig export info"),
    };
    m_wallet = monero::close_wallet(String::from(&old.orid)).await;
    if !m_wallet {
        error!("error closing wallet {}", &old.orid);
    }
}
