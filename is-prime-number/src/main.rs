#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;
use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;

#[derive(Serialize)]
struct NumberResponse {
    numebr: u64,
    is_prime: bool,
    execution_time: u128,
}

#[get("/")]
fn index() -> &'static str {
    "This is a test"
}

#[rocket::main]
async fn main() {
    let mut config = rocket::config::Config::default();
    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    let _ = rocket::build()
        .configure(config)
        .mount("/", routes![index])
        .launch()
        .await;
}
