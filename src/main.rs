#![allow(warnings)]

extern crate actix;
extern crate actix_web;
extern crate actix_redis;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate crypto;

mod common;
mod controllers;
mod models;

use std::sync::Arc;
use actix_web::{server, App, http::{self, header, Method}, middleware::{self, session::SessionStorage, cors::Cors}};
use actix_redis::RedisSessionBackend;

use controllers::user;
use controllers::work_record;
use controllers::error;
use common::state::AppState;

fn main() {

    let config = dotenv::var("CONFIG").expect("CONFIG must be set in .env file");

    if config == "dev" {
        std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
        std::env::set_var("RUST_BACKTRACE", "1");

        env_logger::init();
    }

    let actix_sys = actix::System::new("partner");

    server::new(|| {
            App::with_state(AppState::new())
                .middleware(middleware::Logger::default())
                .middleware(SessionStorage::new(
                    RedisSessionBackend::new("127.0.0.1:6379", &[0;32])
                ))
                .prefix("/api")
                .configure(|app| {
                    Cors::for_app(app)
                    .allowed_origin("http://localhost:4200")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .resource("/register", |r| {
                        r.method(http::Method::POST).with2(user::register)
                    })
                    .resource("/login", |r| {
                        r.method(http::Method::POST).with2(user::login)
                    })
                    .resource("/user/update", |r| {
                        r.method(http::Method::PUT).with2(user::update)
                    })
                    .resource("/user/delete", |r| {
                        r.method(http::Method::DELETE).with2(user::delete)
                    })
                    .resource("/modify-password", |r| {
                        r.method(http::Method::PUT).with2(user::modify_password)
                    })
                    .resource("/work-record/create", |r| {
                        r.method(http::Method::POST).with2(work_record::create)
                    })
                    .resource("/work-record/update", |r| {
                        r.method(http::Method::PUT).with2(work_record::update)
                    })
                    .resource("/work-record/get-records", |r| {
                        r.method(http::Method::GET).with2(work_record::get_records)
                    })
                    .resource("/work-record/get-record", |r| {
                        r.method(http::Method::GET).with2(work_record::get_record)
                    })
                    .resource("/work-record/delete", |r| {
                        r.method(http::Method::DELETE).with2(work_record::delete)
                    })
                    .register()
                })
                .default_resource(|r| {
                        r.f(error::not_found)
                })
        })
        .bind("127.0.0.1:8888")
        .expect("can't bind to port 8888")
        .start();

    println!("server is listening on port 8888 !");

    actix_sys.run();
}
