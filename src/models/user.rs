use crate::schemas::user::user;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{Output, ToSql},
    sql_types::Integer,
    Queryable, Selectable,
};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::diesel::Insertable;

#[repr(i32)]
#[derive(AsExpression, FromSqlRow, Deserialize, Serialize, Debug)]
#[diesel(sql_type = Integer)]
pub enum Gender {
    Male = 1,
    Female = 2,
}

impl ToSql<Integer, diesel::pg::Pg> for Gender
where
    i32: ToSql<Integer, diesel::pg::Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        let v = match self {
            Gender::Male => 1,
            Gender::Female => 2,
        };
        <i32 as ToSql<Integer, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

impl<DB> FromSql<Integer, DB> for Gender
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Gender::Male),
            2 => Ok(Gender::Female),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

#[derive(Queryable, Insertable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = user)]
#[serde(crate = "rocket::serde")]
pub struct User {
    id: i32,
    name: String,
    gender: Gender,
    age: i32,
    password: String,
}

// impl User {
//     pub fn new(id: i32, name: String, gender: Gender, age: i32) -> Self {
//         Self {
//             id,
//             name,
//             gender,
//             age,
//         }
//     }
// }
