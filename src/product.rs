use crate::models::*;
use crate::schema;
use crate::utils;
use diesel::prelude::*;
use log::{debug, error, info};

#[derive(Debug)]
enum UpdateType {
    InStock,
    Description,
    Name,
    Price,
    Quantity,
}

impl UpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            UpdateType::InStock => 0,
            UpdateType::Description => 1,
            UpdateType::Name => 2,
            UpdateType::Price => 3,
            UpdateType::Quantity => 4,
        }
    }
}

/// Create a skeleton for the product
pub async fn create(v_id: String) -> Product {
    use crate::schema::products;
    let connection = &mut utils::establish_pgdb_connection().await;
    let pid: String = utils::generate_rnd();
    let new_product = NewProduct {
        pid: &pid,
        v_id: &v_id,
        in_stock: &false,
        p_description: "",
        p_name: "",
        p_price: &0,
        qty: &0,
    };
    debug!("insert product: {:?}", new_product);
    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(connection)
        .expect("error saving new product")
}

/// Lookup product
pub async fn find(r_pid: String) -> Product {
    use self::schema::products::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let results = products
        .filter(schema::products::pid.eq(r_pid))
        .load::<Product>(connection);
    match results {
        Ok(mut r) => {
            info!("found product");
            if &r.len() > &0 {
                r.remove(0)
            } else {
                Default::default()
            }
        }
        _ => {
            error!("error finding product");
            Default::default()
        }
    }
}

pub async fn modify(_id: String, data: String, update_type: i32) -> Product {
    use self::schema::products::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    // TODO: this isn't right. The product should automatically
    // get updated based on the qty. Qty should be updated according
    // to settled orders
    if update_type == UpdateType::InStock.value() {
        info!("modify product active status");
        let m = diesel::update(products.find(_id))
            .set(in_stock.eq(true))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Description.value() {
        info!("modify product description");
        let m = diesel::update(products.find(_id))
            .set(p_description.eq(data))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Name.value() {
        info!("modify product name");
        let m = diesel::update(products.find(_id))
            .set(p_name.eq(data))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Price.value() {
        info!("modify product price");
        let price_data = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(products.find(_id))
            .set(p_price.eq(price_data))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Quantity.value() {
        info!("modify product quantity");
        let amt = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(products.find(_id))
            .set(qty.eq(amt))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    }
    Default::default()
}

pub async fn find_all(_v_id: String) -> Vec<Product> {
    use self::schema::products::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let results = products
        .filter(schema::products::v_id.eq(_v_id))
        .load::<Product>(connection);
    match results {
        Ok(r) => {
            info!("found vendor products");
            r
        }
        _ => {
            error!("error finding vendor products");
            let v: Vec<Product> = Vec::new();
            v
        }
    }
}
