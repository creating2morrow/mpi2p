// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        c_name -> Varchar,
        c_pgp -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        o_date -> Int4,
        o_hash -> Varchar,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        p_name -> Varchar,
        p_pgp -> Varchar,
        p_price -> Int4,
        qty -> Int4,
    }
}

diesel::table! {
    vendors (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        active -> Bool,
    }
}

diesel::joinable!(orders -> customers (id));
diesel::joinable!(orders -> products (id));
diesel::joinable!(products -> vendors (id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    orders,
    products,
    vendors,
);
