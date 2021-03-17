table! {
    notifications (id) {
        id -> Int4,
        price -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        created_at -> Timestamp,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    notifications,
    users,
);
