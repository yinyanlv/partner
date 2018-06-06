use actix_web::{HttpRequest, Responder};


pub fn register(req: HttpRequest) -> impl Responder {

    "regiser"
}

pub fn login(req: HttpRequest) -> impl Responder {

    "login"
}

pub fn update(req: HttpRequest) -> impl Responder {

    "update"
}

pub fn reset(req: HttpRequest) -> impl Responder {

    "reset"
}