table! {
    notifications (id) {
        id -> Int4,
        price -> Float8,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        created_at -> Timestamp,
        hash -> Varchar,
    }
}

joinable!(notifications -> users (user_id));

allow_tables_to_appear_in_same_query!(
    notifications,
    users,
);
