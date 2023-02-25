use crate::logger;
use crate::models::*;
use crate::schema;
use crate::utils;
use diesel::prelude::*;

#[derive(Debug)]
pub enum ProductUpdateType {
    InStock,
    Description,
    Name,
    Price,
}

impl ProductUpdateType {
    pub fn value(&self) -> i32 {
        match *self {
            ProductUpdateType::InStock => 0,
            ProductUpdateType::Description => 1,
            ProductUpdateType::Name => 2,
            ProductUpdateType::Price => 3,
        }
    }
}

pub async fn create_new_product(v_id: String) -> Product {
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
    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(connection)
        .expect("Error saving new product")
}

pub async fn modify_product(_id: String, data: String, update_type: i32) -> Product {
    use self::schema::products::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    // TODO: this isn't right. The product should automatically
    // get updated based on the qty. Qty should be updated according
    // to settled orders
    if update_type == ProductUpdateType::InStock.value() {
        logger::log(logger::LogLevel::INFO, "Modify product active status.").await;
        let m = diesel::update(products.find(_id))
            .set(in_stock.eq(true))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == ProductUpdateType::Description.value() {
        logger::log(logger::LogLevel::INFO, "Modify product description.").await;
        let m = diesel::update(products.find(_id))
            .set(p_description.eq(data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == ProductUpdateType::Name.value() {
        logger::log(logger::LogLevel::INFO, "Modify product name.").await;
        let m = diesel::update(products.find(_id))
            .set(p_name.eq(data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    } else if update_type == ProductUpdateType::Price.value() {
        logger::log(logger::LogLevel::INFO, "Modify product price.").await;
        let price_data = match data.parse::<i64>() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let m = diesel::update(products.find(_id))
            .set(p_price.eq(price_data))
            .get_result::<Product>(connection);
        match m {
            Ok(m) => m,
            Err(_e) => Default::default(),
        };
    }
    Default::default()
}

pub async fn find_vendor_products(_v_id: String) -> Vec<Product> {
    use self::schema::products::dsl::*;
    let connection = &mut utils::establish_pgdb_connection().await;
    let results = products
        .filter(schema::products::v_id.eq(_v_id))
        .load::<Product>(connection);
    match results {
        Ok(r) => {
            logger::log(logger::LogLevel::INFO, "Found vendor products.").await;
            r
        }
        _ => {
            logger::log(logger::LogLevel::ERROR, "Error finding vendor products.").await;
            let v: Vec<Product> = Vec::new();
            v
        }
    }
}
