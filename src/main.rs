#[macro_use]
extern crate rocket;

use mpi2p::*;

// The only changes in here should be mounting new controller methods

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    utils::start_up().await;
    rocket::build()
        .mount("/", routes![controller::login])
        .mount("/customer", routes![controller::get_customer /*controller::update_customer*/])
        // .mount("/vendor", routes![controller::get_vendor, controller::update_vendor])
        // .mount("/product", routes![controller::create_product, controller::update_product])
        // .mount("/products", routes![controller::get_vendor_products])
        // .mount("/order", routes![controller::initialize_order, controller::update_order])
        // .mount("/orders", routes![controller::get_orders])
        // .mount("/xmr", routes![controller::get_version])
        // .mount("/dispute", routes![controller::create_dispute, controller::get_dispute])
}
