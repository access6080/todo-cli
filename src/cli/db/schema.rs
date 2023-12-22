// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> Integer,
        title -> Text,
        is_complete -> Integer,
    }
}
