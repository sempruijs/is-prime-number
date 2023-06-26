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
    execution_time_in_micros: u128,
}

#[get("/")]
fn index() -> &'static str {
    "This is a test"
}

fn is_prime_number(n: u64) -> bool {
    let numbers: Vec<u64> = (2..n).collect();
    for number in numbers {
        if n % number == 0 {
            return false;
        }
    }

    true
}

#[get("/isPrime?<n>")]
fn get_is_prime(n: u64) -> Json<NumberResponse> {
    let now = SystemTime::now();

    Json(NumberResponse {
        numebr: n,
        is_prime: is_prime_number(n),
        execution_time_in_micros: now.elapsed().unwrap().as_micros(),
    })
}

#[rocket::main]
async fn main() {
    let mut config = rocket::config::Config::default();
    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    let _ = rocket::build()
        .configure(config)
        .mount("/", routes![index, get_is_prime])
        .launch()
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime_number(0));
        assert!(is_prime_number(1));
        assert!(is_prime_number(2));
        assert!(is_prime_number(3));
        assert!(!is_prime_number(4));
        assert!(is_prime_number(5));
        assert!(!is_prime_number(6));
        assert!(is_prime_number(7));
        assert!(!is_prime_number(8));
        assert!(!is_prime_number(9));
        assert!(!is_prime_number(10));
    }
}
