// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        c_xmr_address -> Varchar,
        c_name -> Varchar,
        c_pgp -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        c_id -> Nullable<Int4>,
        p_id -> Nullable<Int4>,
        o_xmr_address -> Nullable<Varchar>,
        o_date -> Int4,
        o_deliver_date -> Nullable<Int4>,
        o_ship_date -> Nullable<Int4>,
        o_hash -> Nullable<Varchar>,
        o_msig_prepare -> Nullable<Text>,
        o_msig_make -> Nullable<Text>,
        o_msig_kex -> Nullable<Text>,
        o_msig_kex_boost -> Nullable<Text>,
        o_status -> Nullable<Text>,
        o_quantity -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        v_id -> Nullable<Int4>,
        in_stock -> Bool,
        p_description -> Text,
        p_name -> Varchar,
        p_price -> Int4,
        qty -> Int4,
    }
}

diesel::table! {
    vendors (id) {
        id -> Int4,
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
    customers,
    orders,
    products,
    vendors,
);
