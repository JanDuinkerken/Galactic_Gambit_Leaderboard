// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Int4,
        level_id -> Varchar,
        username -> Varchar,
        seconds -> Float8,
    }
}
