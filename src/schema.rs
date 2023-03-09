// @generated automatically by Diesel CLI.

diesel::table! {
    presences (id) {
        id -> Int4,
        id_str -> Varchar,
        msg_type -> Varchar,
        reason -> Text,
        asset -> Text,
        time -> Text,
    }
}
