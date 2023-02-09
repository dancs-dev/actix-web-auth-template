// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        password -> Text,
        session_id -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    users,
);
