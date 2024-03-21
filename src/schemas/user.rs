// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        gender -> Integer,
        age -> Integer,
        password -> Text,
    }
}
