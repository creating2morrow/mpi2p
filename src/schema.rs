// @generated automatically by Diesel CLI.

diesel::table! {
    authorizations (aid) {
        aid -> Varchar,
        created -> Int8,
        rnd -> Varchar,
        xmr_address -> Varchar,
    }
}

diesel::table! {
    customers (cid) {
        cid -> Varchar,
        c_xmr_address -> Varchar,
        c_name -> Varchar,
        c_pgp -> Varchar,
    }
}

diesel::table! {
    orders (orid) {
        orid -> Varchar,
        c_id -> Varchar,
        p_id -> Varchar,
        o_xmr_address -> Varchar,
        o_cust_msig_info -> Varchar,
        o_cust_kex_1 -> Varchar,
        o_cust_kex_2 -> Varchar,
        o_cust_kex_3 -> Varchar,
        o_date -> Int8,
        o_deliver_date -> Int8,
        o_ship_date -> Int8,
        o_hash -> Varchar,
        o_msig_prepare -> Text,
        o_msig_make -> Text,
        o_msig_kex_1 -> Text,
        o_msig_kex_2 -> Text,
        o_msig_kex_3 -> Text,
        o_status -> Text,
        o_quantity -> Int8,
        o_vend_kex_1 -> Varchar,
        o_vend_kex_2 -> Varchar,
        o_vend_kex_3 -> Varchar,
    }
}

diesel::table! {
    products (pid) {
        pid -> Varchar,
        v_id -> Varchar,
        in_stock -> Bool,
        p_description -> Text,
        p_name -> Varchar,
        p_price -> Int8,
        qty -> Int8,
    }
}

diesel::table! {
    vendors (vid) {
        vid -> Varchar,
        v_xmr_address -> Varchar,
        v_name -> Varchar,
        v_description -> Text,
        v_pgp -> Text,
        active -> Bool,
    }
}

diesel::joinable!(orders -> customers (c_id));
diesel::joinable!(orders -> products (p_id));
diesel::joinable!(products -> vendors (v_id));

diesel::allow_tables_to_appear_in_same_query!(
    authorizations,
    customers,
    orders,
    products,
    vendors,
);
