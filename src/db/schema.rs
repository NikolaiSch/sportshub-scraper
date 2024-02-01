// @generated automatically by Diesel CLI.

diesel::table! {
    stream (id) {
        id -> Nullable<Integer>,
        home -> Text,
        away -> Text,
        start_time -> Timestamp,
        league -> Text,
        country -> Text,
        url -> Text,
        stream_link -> Text,
        sport -> Text,
    }
}
