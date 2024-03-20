use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::{models::user::User, schemas};

use super::connections::establish_connsection;

pub async fn create_user_to_db(user: User) -> User {
    let mut conn = establish_connsection().await;
    diesel::insert_into(schemas::user::user::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .expect("Error saving new user")
}
