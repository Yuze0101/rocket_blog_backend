use crate::{apis::register::create_user_to_db, models::user::User};
use rocket::serde::json::Json;

#[post("/", data = "<user>")]
pub async fn register(user: Json<User>) {
    println!("{:?}", user);
    let user = user.into_inner();
    println!("{:?}", user);
    create_user_to_db(user).await;
}
