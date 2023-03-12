use crate::{models, utils};
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
pub struct XmrRpcImportParams {
    pub info: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcSignMultisigParams {
    pub tx_data_hex: String,
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
pub struct XmrRpcImportRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcImportParams,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct XmrRpcSignMultisigRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: XmrRpcSignMultisigParams,
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

#[derive(Serialize, Deserialize, Debug)]
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
pub struct XmrRpcExportResult {
    pub info: String,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcImportResult {
    pub n_outputs: u8,
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcSignMultisigResult {
    pub tx_hash_list: Vec<String>,
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

#[derive(Serialize, Deserialize, Debug)]
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
                address: utils::empty_string(),
                multisig_info: utils::empty_string(),
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
                address: utils::empty_string(),
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcExportResponse {
    pub result: XmrRpcExportResult,
}

impl Default for XmrRpcExportResponse {
    fn default() -> Self {
        XmrRpcExportResponse {
            result:
            XmrRpcExportResult {
                info: utils::empty_string(),
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcImportResponse {
    pub result: XmrRpcImportResult,
}

impl Default for XmrRpcImportResponse {
    fn default() -> Self {
        XmrRpcImportResponse {
            result:
            XmrRpcImportResult {
                n_outputs: 0,
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct XmrRpcSignMultisigResponse {
    pub result: XmrRpcSignMultisigResult,
}

impl Default for XmrRpcSignMultisigResponse {
    fn default() -> Self {
        XmrRpcSignMultisigResponse {
            result:
            XmrRpcSignMultisigResult {
                tx_hash_list:Vec::new(),
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
            address: utils::empty_string(),
            name: utils::empty_string(),
            pgp: utils::empty_string(),
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
            address: utils::empty_string(),
            description: utils::empty_string(),
            name: utils::empty_string(),
            pgp: utils::empty_string(),
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
            pid: utils::empty_string(),
            in_stock: false,
            description: utils::empty_string(),
            name: utils::empty_string(),
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
    pub aid: String,
    pub created: i64,
    pub cvid: String,
    pub data: String,
    pub token: String,
}

impl Default for GetAuthResponse {
    fn default() -> Self {
        GetAuthResponse {
            address: utils::empty_string(),
            aid: utils::empty_string(),
            created: 0,
            cvid: utils::empty_string(),
            data: utils::empty_string(),
            token: utils::empty_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetDisputeResponse {
    pub orid: String,
    pub created: i64,
    pub tx_set: String,
}

impl Default for GetDisputeResponse {
    fn default() -> Self {
        GetDisputeResponse {
            orid: utils::empty_string(),
            created: 0,
            tx_set: utils::empty_string(),
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
    pub subaddress: String,
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
            orid: utils::empty_string(),
            pid: utils::empty_string(),
            xmr_address: utils::empty_string(),
            cust_msig_info: utils::empty_string(),
            cust_kex_1: utils::empty_string(),
            cust_kex_2: utils::empty_string(),
            cust_kex_3: utils::empty_string(),
            date: 0,
            deliver_date: 0,
            ship_date: 0,
            hash: utils::empty_string(),
            msig_prepare: utils::empty_string(),
            msig_make: utils::empty_string(),
            msig_kex_1: utils::empty_string(),
            msig_kex_2: utils::empty_string(),
            msig_kex_3: utils::empty_string(),
            subaddress: utils::empty_string(),
            status: utils::empty_string(),
            quantity: 0,
            vend_kex_1: utils::empty_string(),
            vend_kex_2: utils::empty_string(),
            vend_kex_3: utils::empty_string(),
            vend_msig_info: utils::empty_string(),
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
            address: m_customer.xmr_address,
            name: m_customer.name,
            pgp: m_customer.pgp,
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
            aid: m_auth.aid,
            created: m_auth.created,
            cvid: m_auth.cvid,
            data: m_auth.rnd,
            token: m_auth.token,
        }
    }
}

impl GetDisputeResponse {
    pub fn build(m_dispute: models::Dispute) -> Self {
        GetDisputeResponse {
            orid: m_dispute.orid,
            created: m_dispute.created,
            tx_set: m_dispute.tx_set,
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
            subaddress: m_order.o_subaddress,
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
                subaddress: m.o_subaddress,
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
