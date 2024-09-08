// @generated automatically by Diesel CLI.

diesel::table! {
    budget (id) {
        id -> Int4,
        january -> Nullable<Float8>,
        february -> Nullable<Float8>,
        march -> Nullable<Float8>,
        april -> Nullable<Float8>,
        may -> Nullable<Float8>,
        june -> Nullable<Float8>,
        july -> Nullable<Float8>,
        august -> Nullable<Float8>,
        september -> Nullable<Float8>,
        october -> Nullable<Float8>,
        november -> Nullable<Float8>,
        december -> Nullable<Float8>,
    }
}

diesel::table! {
    expenses (id) {
        id -> Int4,
        description -> Text,
        amount -> Float8,
        created_at -> Nullable<Timestamp>,
        update_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pc_stats (stat_id) {
        #[max_length = 255]
        stat_id -> Varchar,
        total_swap -> Float8,
        free_swap -> Float8,
        total_memory -> Float8,
        free_memory -> Float8,
        total_disk -> Float8,
        free_disk -> Float8,
    }
}

diesel::table! {
    session_tokens (token) {
        token -> Varchar,
        username -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    budget,
    expenses,
    pc_stats,
    session_tokens,
    users,
);
