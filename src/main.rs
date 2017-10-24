#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate rocket;

use std::net::{SocketAddr, IpAddr};

#[get("/")]
fn ip(remote_address: SocketAddr) -> String {

    format!("{}", IpAddr::from(remote_address.ip()))
}

fn main() {
    let config = rocket::config::Config::build(rocket::config::Environment::Staging)
        .port(8888)
        .finalize()
        .expect("Could not create config");

    rocket::custom(config, false)
        .mount("/", routes![ip])
        .launch();
}

mod tests;
