use crate::logger;
use crate::models::*;
use crate::schema;
use crate::utils;
use diesel::prelude::*;

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
    logger::log(logger::LogLevel::DEBUG,
        &format!("insert product: {:?}", new_product)).await;
    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(connection)
        .expect("error saving new product")
}

pub async fn modify(_id: String, data: String, update_type: i32) -> Product {
    use self::schema::products::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    // TODO: this isn't right. The product should automatically
    // get updated based on the qty. Qty should be updated according
    // to settled orders
    if update_type == UpdateType::InStock.value() {
        logger::log(logger::LogLevel::INFO, "modify product active status").await;
        let m = diesel::update(products.find(_id))
            .set(in_stock.eq(true))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Description.value() {
        logger::log(logger::LogLevel::INFO, "modify product description").await;
        let m = diesel::update(products.find(_id))
            .set(p_description.eq(data))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Name.value() {
        logger::log(logger::LogLevel::INFO, "modify product name").await;
        let m = diesel::update(products.find(_id))
            .set(p_name.eq(data))
            .get_result::<Product>(connection);
        return match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == UpdateType::Price.value() {
        logger::log(logger::LogLevel::INFO, "modify product price").await;
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
        logger::log(logger::LogLevel::INFO, "modify product quantity").await;
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
            logger::log(logger::LogLevel::INFO, "found vendor products").await;
            r
        }
        _ => {
            logger::log(logger::LogLevel::ERROR, "error finding vendor products").await;
            let v: Vec<Product> = Vec::new();
            v
        }
    }
}
