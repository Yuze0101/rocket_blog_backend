use crate::{
    apis::register::{create_user_to_db, get_all_users},
    models::{response::MyResponse, user::User},
};
use rocket::serde::json::Json;

#[post("/", data = "<user>")]
pub async fn register(user: Json<User>) -> Json<MyResponse<User>> {
    let user = user.into_inner();
    match create_user_to_db(user).await {
        Ok(u) => Json(MyResponse::new(
            200,
            true,
            Some("User created successfully".to_string()),
            Some(u),
        )),
        Err(_) => Json(MyResponse::new(
            400,
            false,
            Some("User creation failed".to_string()),
            None,
        )),
    }
}

#[get("/")]
pub async fn test_get_users() -> Json<MyResponse<Vec<User>>> {
    match get_all_users().await {
        Ok(v) => Json(MyResponse::new(
            200,
            true,
            Some("Test get all users success".to_string()),
            Some(v),
        )),
        Err(e) => {
            println!("{:?}", e);
            Json(MyResponse::new(
                400,
                false,
                Some("Test get all users faileds".to_string()),
                None,
            ))
        }
    }
}
