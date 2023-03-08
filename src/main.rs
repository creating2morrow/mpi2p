#[macro_use]
extern crate rocket;
use mpi2p::*;
use log::info;
#[cfg(test)]
mod tests;

// The only changes in here should be mounting new controller methods

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    info!("mpi2p is starting up");
    // postgres required to be up at boot time
    utils::establish_pgdb_connection().await;
    monero::check_rpc_connection().await;
    let env: String = utils::get_release_env().value();
    let dev: String = utils::ReleaseEnvironment::Development.value();
    if env != dev { tokio::spawn(async { i2p::check_connection().await }); }
    info!("{} - mpi2p is online", env);
    rocket::build()
        .mount("/", routes![controller::login])
        .mount("/customer", routes![controller::get_customer, controller::update_customer])
        .mount("/vendor", routes![controller::get_vendor, controller::update_vendor])
        .mount("/product", routes![controller::create_product, controller::update_product])
        .mount("/products", routes![controller::get_vendor_products])
        .mount("/order", routes![controller::initialize_order, controller::update_order])
        .mount("/orders", routes![controller::get_orders])
        .mount("/xmr", routes![controller::get_version])
        .mount("/dispute", routes![controller::create_dispute, controller::get_dispute])
}
