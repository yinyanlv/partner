#![allow(warnings)]

extern crate actix;
extern crate actix_web;
// extern crate mysql;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod controllers;
mod common;

use actix_web::{server, App};
use controllers::user;

fn main() {
    let config = dotenv::var("CONFIG").expect("CONFIG must be set in .env file");

    if config == "dev" {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let actix_sys = actix::System::new("partner");

    server::new(|| {
        App::new()
            .prefix("/user")
            .resource("/register", |r| r.f(user::register))
            .resource("/login", |r| r.f(user::login))
            .resource("/update", |r| {
                r.f(user::update);
            })
            .resource("/reset", |r| {
                r.f(user::reset);
            })
    }).bind("127.0.0.1:8888")
        .expect("can't bind to port 8888")
        .start();

    actix_sys.run();
}
