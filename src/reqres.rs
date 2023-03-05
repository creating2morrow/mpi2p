use crate::models;
use serde::{Deserialize, Serialize};
// All http requests and responses are here

// START XMR Structs

// params
#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcVerifyParams {
    pub address: String,
    pub data: String,
    pub signature: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcWalletParams {
    pub filename: String,
    pub language: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcMakeParams {
    pub multisig_info: Vec<String>,
    pub threshold: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcFinalizeParams {
    pub multisig_info: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcBalanceParams {
    pub account_index: u8,
    pub address_indices: Vec<u8>,
    pub all_accounts: bool,
    pub strict: bool,
}
// requests
#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcCreateRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcWalletParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcBalanceRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcBalanceParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcMakeRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcMakeParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcFinalizeRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcFinalizeParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcVerifyRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcVerifyParams,
}
// results
#[derive(Deserialize, Debug)]
pub struct XmrRpcVerifyResult {
    pub good: bool,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcVersionResult {
    pub version: i32,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcFinalizeResult {
    pub address: String,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcPrepareResult {
    pub multisig_info: String,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcMakeResult {
    pub address: String,
    pub multisig_info: String,
}

#[derive(Deserialize, Debug)]
pub struct SubAddressInfo {
    pub account_index: u8,
    pub address_index: u8,
    pub address: String,
    pub balance: u128,
    pub unlocked_balance: u128,
    pub label: String,
    pub num_unspent_outputs: u8,
    pub time_to_unlock: u128,
    pub blocks_to_unlock: u128,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcBalanceResult {
    pub balance: u128,
    pub unlocked_balance: u128,
    pub multisig_import_needed: bool,
    pub time_to_unlock: u128,
    pub blocks_to_unlock: u128,
    pub per_subaddress: Vec<SubAddressInfo>,

}
// responses
#[derive(Deserialize, Debug)]
pub struct XmrRpcVerifyResponse {
    pub result: XmrRpcVerifyResult,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct XmrApiVersionResponse {
    pub version: i32,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcVersionResponse {
    pub result: XmrRpcVersionResult,
}

impl Default for XmrRpcVersionResponse {
    fn default() -> Self {
        XmrRpcVersionResponse {
            result: XmrRpcVersionResult { version: 0 }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcPrepareResponse {
    pub result: XmrRpcPrepareResult,
}

impl Default for XmrRpcPrepareResponse {
    fn default() -> Self {
        XmrRpcPrepareResponse {
            result:
            XmrRpcPrepareResult {
                multisig_info: String::from("")
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcBalanceResponse {
    pub result: XmrRpcBalanceResult,
}

impl Default for XmrRpcBalanceResponse {
    fn default() -> Self {
        XmrRpcBalanceResponse {
            result:
            XmrRpcBalanceResult {
                balance: 0,
                unlocked_balance: 0,
                multisig_import_needed: false,
                time_to_unlock: 0,
                blocks_to_unlock: 0,
                per_subaddress: Vec::new()
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcMakeResponse {
    pub result: XmrRpcMakeResult,
}

impl Default for XmrRpcMakeResponse {
    fn default() -> Self {
        XmrRpcMakeResponse {
            result:
            XmrRpcMakeResult {
                address: String::from(""),
                multisig_info: String::from(""),
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcFinalizeResponse {
    pub result: XmrRpcFinalizeResult,
}

impl Default for XmrRpcFinalizeResponse {
    fn default() -> Self {
        XmrRpcFinalizeResponse {
            result:
            XmrRpcFinalizeResult {
                address: String::from(""),
            }
        }
    }
}
// END XMR Structs

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetCustomerResponse {
    pub address: String,
    pub name: String,
    pub pgp: String,
}

impl Default for GetCustomerResponse {
    fn default() -> Self {
        GetCustomerResponse {
            address: String::from(""),
            name: String::from(""),
            pgp: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVendorResponse {
    pub active: bool,
    pub address: String,
    pub description: String,
    pub name: String,
    pub pgp: String,
}

impl Default for GetVendorResponse {
    fn default() -> Self {
        GetVendorResponse {
            active: false,
            address: String::from(""),
            description: String::from(""),
            name: String::from(""),
            pgp: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetProductResponse {
    pub pid: String,
    pub in_stock: bool,
    pub description: String,
    pub name: String,
    pub price: i64,
    pub qty: i64,
}

impl Default for GetProductResponse {
    fn default() -> Self {
        GetProductResponse {
            pid: String::from(""),
            in_stock: false,
            description: String::from(""),
            name: String::from(""),
            price: 0,
            qty: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVendorProductsResponse {
    pub products: Vec<GetProductResponse>,
}

impl Default for GetVendorProductsResponse {
    fn default() -> Self {
        GetVendorProductsResponse {
            products: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAuthResponse {
    pub address: String,
    pub created: i64,
    pub data: String,
}

impl Default for GetAuthResponse {
    fn default() -> Self {
        GetAuthResponse {
            address: String::from(""),
            created: 0,
            data: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetOrderResponse {
    pub orid: String,
    pub pid: String,
    pub xmr_address: String,
    pub cust_msig_info: String,
    pub cust_kex_1: String,
    pub cust_kex_2: String,
    pub cust_kex_3: String,
    pub date: i64,
    pub deliver_date: i64,
    pub ship_date: i64,
    pub hash: String,
    pub msig_prepare: String,
    pub msig_make: String,
    pub msig_kex_1: String,
    pub msig_kex_2: String,
    pub msig_kex_3: String,
    pub status: String,
    pub quantity: i64,
    pub vend_kex_1: String,
    pub vend_kex_2: String,
    pub vend_kex_3: String,
    pub vend_msig_info: String,
}

impl Default for GetOrderResponse {
    fn default() -> Self {
        GetOrderResponse {
            orid: String::from(""),
            pid: String::from(""),
            xmr_address: String::from(""),
            cust_msig_info: String::from(""),
            cust_kex_1: String::from(""),
            cust_kex_2: String::from(""),
            cust_kex_3: String::from(""),
            date: 0,
            deliver_date: 0,
            ship_date: 0,
            hash: String::from(""),
            msig_prepare: String::from(""),
            msig_make: String::from(""),
            msig_kex_1: String::from(""),
            msig_kex_2: String::from(""),
            msig_kex_3: String::from(""),
            status: String::from(""),
            quantity: 0,
            vend_kex_1: String::from(""),
            vend_kex_2: String::from(""),
            vend_kex_3: String::from(""),
            vend_msig_info: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetOrdersResponse {
    pub orders: Vec<GetOrderResponse>,
}

impl Default for GetOrdersResponse {
    fn default() -> Self {
        GetOrdersResponse {
            orders: Vec::new(),
        }
    }
}

// START response builders
impl GetCustomerResponse {
    pub fn build(m_customer: models::Customer) -> Self {
        GetCustomerResponse {
            address: m_customer.c_xmr_address,
            name: m_customer.c_name,
            pgp: m_customer.c_pgp,
        }
    }
}

impl GetVendorResponse {
    pub fn build(m_vendor: models::Vendor) -> Self {
        GetVendorResponse {
            active: m_vendor.active,
            address: m_vendor.v_xmr_address,
            description: m_vendor.v_description,
            name: m_vendor.v_name,
            pgp: m_vendor.v_pgp,
        }
    }
}

impl GetAuthResponse {
    pub fn build(m_auth: models::Authorization) -> Self {
        GetAuthResponse {
            address: m_auth.xmr_address,
            data: m_auth.rnd,
            created: m_auth.created,
        }
    }
}

impl GetProductResponse {
    pub fn build(m_product: models::Product) -> Self {
        GetProductResponse {
            pid: m_product.pid,
            in_stock: m_product.in_stock,
            description: m_product.p_description,
            name: m_product.p_name,
            price: m_product.p_price,
            qty: m_product.qty,
        }
    }
}

impl GetVendorProductsResponse {
    pub fn build(m_products: Vec<models::Product>) -> Self {
        let mut v_res: Vec<GetProductResponse> = Vec::new();
        for m in m_products {
            let p_res: GetProductResponse = GetProductResponse {
                pid: m.pid,
                in_stock: m.in_stock,
                description: m.p_description,
                name: m.p_name,
                price: m.p_price,
                qty: m.qty,
            };
            v_res.push(p_res);
        }
        GetVendorProductsResponse { products: v_res }
    }
}

impl GetOrderResponse {
    pub fn build(pid: String, m_order: models::Order) -> Self {
        GetOrderResponse {
            orid: m_order.orid,
            pid,
            xmr_address: m_order.o_xmr_address,
            cust_msig_info: m_order.o_cust_msig_info,
            cust_kex_1: m_order.o_cust_kex_1,
            cust_kex_2: m_order.o_cust_kex_2,
            cust_kex_3: m_order.o_cust_kex_3,
            date: m_order.o_date,
            deliver_date: m_order.o_deliver_date,
            ship_date: m_order.o_ship_date,
            hash: m_order.o_hash,
            msig_prepare: m_order.o_msig_prepare,
            msig_make: m_order.o_msig_make,
            msig_kex_1: m_order.o_msig_kex_1,
            msig_kex_2: m_order.o_msig_kex_2,
            msig_kex_3: m_order.o_msig_kex_3,
            status: m_order.o_status,
            quantity: m_order.o_quantity,
            vend_kex_1: m_order.o_vend_kex_1,
            vend_kex_2: m_order.o_vend_kex_2,
            vend_kex_3: m_order.o_vend_kex_3,
            vend_msig_info: m_order.o_vend_msig_info,
        }
    }
}

impl GetOrdersResponse {
    pub fn build(m_orders: Vec<models::Order>) -> Self {
        let mut v_res: Vec<GetOrderResponse> = Vec::new();
        for m in m_orders {
            let o_res: GetOrderResponse = GetOrderResponse {
                orid: m.orid,
                pid: m.p_id,
                xmr_address: m.o_xmr_address,
                cust_msig_info: m.o_cust_msig_info,
                cust_kex_1: m.o_cust_kex_1,
                cust_kex_2: m.o_cust_kex_2,
                cust_kex_3: m.o_cust_kex_3,
                date: m.o_date,
                deliver_date: m.o_deliver_date,
                ship_date: m.o_ship_date,
                hash: m.o_hash,
                msig_prepare: m.o_msig_prepare,
                msig_make: m.o_msig_make,
                msig_kex_1: m.o_msig_kex_1,
                msig_kex_2: m.o_msig_kex_2,
                msig_kex_3: m.o_msig_kex_3,
                status: m.o_status,
                quantity: m.o_quantity,
                vend_kex_1: m.o_vend_kex_1,
                vend_kex_2: m.o_vend_kex_2,
                vend_kex_3: m.o_vend_kex_3,
                vend_msig_info: m.o_vend_msig_info,
            };
            v_res.push(o_res);
        }
            GetOrdersResponse { orders: v_res }
    }
}
// END response builders
