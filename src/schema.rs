// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Integer,
        level_id -> Integer,
        username -> Text,
        seconds -> Float,
    }
}
