// @generated automatically by Diesel CLI.

diesel::table! {
    locations (id) {
        id -> Uuid,
        title -> Text,
        prompt -> Text,
    }
}

diesel::table! {
    votes (id) {
        id -> Uuid,
        user_agent -> Text,
        agrees -> Bool,
        comment -> Nullable<Text>,
        location_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(votes -> locations (location_id));

diesel::allow_tables_to_appear_in_same_query!(
    locations,
    votes,
);
