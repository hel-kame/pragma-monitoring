// @generated automatically by Diesel CLI.

diesel::table! {
    future_entry (data_id) {
        network -> Varchar,
        pair_id -> Varchar,
        data_id -> Varchar,
        block_hash -> Varchar,
        block_number -> BigInt,
        block_timestamp -> Nullable<Timestamp>,
        transaction_hash -> Varchar,
        price -> Numeric,
        timestamp -> Nullable<Timestamp>,
        publisher -> Varchar,
        source -> Varchar,
        volume -> Numeric,
        expiration_timestamp -> Nullable<Timestamp>,
        _cursor -> BigInt,
    }
}

diesel::table! {
    spot_entry (data_id) {
        network -> Varchar,
        pair_id -> Varchar,
        data_id -> Varchar,
        block_hash -> Varchar,
        block_number -> BigInt,
        block_timestamp -> Nullable<Timestamp>,
        transaction_hash -> Varchar,
        price -> Numeric,
        timestamp -> Nullable<Timestamp>,
        publisher -> Varchar,
        source -> Varchar,
        volume -> Numeric,
        _cursor -> BigInt,
    }
}
