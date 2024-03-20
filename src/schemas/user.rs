// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        name -> Text,
        age -> Int4,
        gender -> Int4,
    }
}
