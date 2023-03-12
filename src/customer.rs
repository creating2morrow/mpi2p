// // Customer repo/service layer
use crate::{auth, db, models::*, monero, utils};
use log::{debug, error, info};

// enum UpdateType {
//     Name,
//     Pgp,
// }

// impl UpdateType {
//     pub fn value(&self) -> i32 {
//         match *self {
//             UpdateType::Name => 2,
//             UpdateType::Pgp => 3,
//         }
//     }
// }

/// Create a new customer
fn create(address: &String) -> Customer {
    let f_cid: String = format!("cust{}", utils::generate_rnd());
    let new_customer = Customer {
        cid: String::from(&f_cid),
        xmr_address: String::from(address),
        name: utils::empty_string(),
        pgp: utils::empty_string(),
    };
    debug!("insert customer: {:?}", &new_customer);
    let s = db::Interface::open();
    let k = &new_customer.cid;
    db::Interface::write(&s.env, &s.handle, k, &Customer::to_db(&new_customer));
    new_customer
}

/// Authorization lookup for recurring requests
pub fn find(cid: &String) -> Customer {
    let s = db::Interface::open();
    let r = db::Interface::read(&s.env, &s.handle, &String::from(cid));
    if r == utils::empty_string() {
        return Default::default()
    }
    Customer::from_db(String::from(cid), r)
}

/// Performs the signature verfication against stored auth
pub async fn verify_login(
    address: String,
    aid: String,
    cvid: String,
    signature: String,
) -> Authorization {
    let f_auth: Authorization = auth::find(&aid);
    if f_auth.xmr_address == utils::empty_string() {
        return auth::create(&address);
    }
    let data: String = String::from(&f_auth.rnd);
    let sig_address: String =
        monero::verify_signature(String::from(&address), data, String::from(&signature)).await;
    if sig_address == utils::ApplicationErrors::LoginError.value() {
        return f_auth;
    }
    let f_cust: Customer = find(&cvid);
    if f_cust.xmr_address == utils::empty_string() {
        info!("creating new customer");
        let c: Customer = create(&address);
        // update auth with cvid
        let u_auth = Authorization::update_cvid(f_auth, String::from(&c.cid));
        let s = db::Interface::open();
        db::Interface::delete(&s.env, &s.handle, &u_auth.aid);
        db::Interface::write(&s.env, &s.handle, &u_auth.aid, &Authorization::to_db(&u_auth));
        return u_auth
    }
    error!("error creating customer");
    Default::default()
}

// // /// Update customer information
// // pub async fn modify(_id: String, data: String, update_type: i32) -> Customer {
// //     use self::schema::customers::dsl::*;
// //     let connection = &mut utils::establish_pgdb_connection().await;
// //     if update_type == UpdateType::Name.value() {
// //         info!("modify customer name");
// //         let m = diesel::update(customers.find(_id))
// //             .set(c_name.eq(data))
// //             .get_result::<Customer>(connection);
// //         return match m {
// //             Ok(m) => m,
// //             Err(_e) => Default::default(),
// //         }
// //     } else if update_type == UpdateType::Pgp.value() {
// //         info!("modify customer pgp");
// //         let m = diesel::update(customers.find(_id))
// //             .set(c_pgp.eq(data))
// //             .get_result::<Customer>(connection);
// //         return match m {
// //             Ok(m) => m,
// //             Err(_e) => Default::default(),
// //         }
// //     }
// //     Default::default()
// // }
