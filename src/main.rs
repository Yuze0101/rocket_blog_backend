#[macro_use]
extern crate rocket;
mod apis;
mod models;
mod routes;
mod schemas;
mod utils;

use std::net::Ipv4Addr;

use rocket::Config;
use routes::register::{register, test_get_users};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::debug_default()
    };
    rocket::build()
        .configure(config)
        .mount("/", routes![index])
        .mount("/register", routes![register, test_get_users])
}
