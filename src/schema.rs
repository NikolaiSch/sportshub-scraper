// @generated automatically by Diesel CLI.

diesel::table! {
    stream (id) {
        id -> Nullable<Integer>,
        home -> Text,
        away -> Text,
        start_time -> Text,
        league -> Text,
        country -> Text,
    }
}
