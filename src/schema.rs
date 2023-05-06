// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Int4,
        level_id -> Int4,
        username -> Varchar,
        seconds -> Float8,
    }
}
