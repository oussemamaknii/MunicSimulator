// @generated automatically by Diesel CLI.

diesel::table! {
    presences (id) {
        id -> Integer,
        id_str -> Varchar,
        r#type -> Varchar,
        reason -> Text,
        asset -> Text,
        time -> Text,
        connection_id-> Integer,
        fullreason-> Text,
        cs-> Text,
        ip-> Text,
        protocol-> Text,
    }
}
