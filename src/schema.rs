// @generated automatically by Diesel CLI.

diesel::table! {
    indexers (id) {
        id -> Uuid,
        status -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        process_id -> Nullable<Int8>,
        target_url -> Varchar,
    }
}

diesel::table! {
    storage (id) {
        id -> Uuid,
        network -> Varchar,
        data_type -> Varchar,
        block_hash -> Varchar,
        block_number -> Int8,
        block_timestamp -> Timestamp,
        transaction_hash -> Varchar,
        source -> Nullable<Varchar>,
        price -> Nullable<Float4>,
        pair_id -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(indexers, storage,);
