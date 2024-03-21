use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::{models::user::User, schemas};

use super::connections::establish_connsection;

pub async fn create_user_to_db(user: User) -> Result<User, diesel::result::Error> {
    let mut conn = establish_connsection().await;
    diesel::insert_into(schemas::user::user::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
}

pub async fn get_all_users() -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = establish_connsection().await;
    let users = schemas::user::user::table;

    users.load::<User>(&mut conn).await
}
