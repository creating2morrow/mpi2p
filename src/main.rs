#[macro_use] extern crate rocket;

pub mod reqres;

#[cfg(test)] mod tests;

/*
 TODO:
   1)  get_customer
   2)  update_customer
   3)  create_product
   4)  get_product
   5)  update_product
   6)  create_vendor
   7)  get_vendor
   8)  update_vendor
   9)  create_order
   10) get_order
   11) update_order
   12) create_customer
   13) msig APIs
   14) sign / verify API for login
   15) testing
*/

#[get("/version")]
async fn xmr() -> String {
    let client = reqwest::Client::new();
    let net = "http://127.0.0.1:38083/json_rpc";
    let req = reqres::RpcRequest { 
        jsonrpc: "2.0".to_string(), 
        id: "0".to_string(), 
        method: "get_version".to_string()
    };
    match client
        .post(net)
        .json(&req)
        .send()
        .await
    {
        Ok(response) => {
            let res = response.json::<reqres::RpcResponse>().await;
            match res {
                Ok(res) => format!(
                    "{{ \"version\": {} }}", res.result.version
                    ),
                _=> "error".to_string()
            }
        }
        Err(_e) => {
            "error".to_string()
        }
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/xmr", routes![xmr])
}
