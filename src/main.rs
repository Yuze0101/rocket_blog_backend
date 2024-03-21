#[macro_use]
extern crate rocket;
mod apis;
mod models;
mod routes;
mod schemas;
mod utils;

use routes::register::{register, test_get_users};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/register", routes![register, test_get_users])
}
