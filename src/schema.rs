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
        o_xmr_address -> Nullable<Varchar>,
        o_date -> Int8,
        o_deliver_date -> Int8,
        o_ship_date -> Int8,
        o_hash -> Nullable<Varchar>,
        o_msig_prepare -> Nullable<Text>,
        o_msig_make -> Nullable<Text>,
        o_msig_kex -> Nullable<Text>,
        o_msig_kex_boost -> Nullable<Text>,
        o_status -> Nullable<Text>,
        o_quantity -> Int8,
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
