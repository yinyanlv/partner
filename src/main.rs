extern crate actix_web;

use actix_web::{server, App, HttpRequest, HttpResponse};

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8888")
        .expect("can't bing to port 8888")
        .run();
}

fn index(req: HttpRequest) -> &'static str {
    "Hello world"
}