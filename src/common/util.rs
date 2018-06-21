use crypto::md5::Md5;
use crypto::digest::Digest;
use rand::{thread_rng, Rng};
use actix_web::HttpRequest;
use actix_web::middleware::session::RequestSession;

use common::state::AppState;
use models::user::RawUser;

pub fn random_string(limit: usize) -> String {

    thread_rng().gen_ascii_chars().take(limit).collect()
}

pub fn md5_encode(text: &str) -> String {

    let mut sh = Md5::new();

    sh.input_str(text);
    sh.result_str().to_string()
}

pub fn is_unauthorized(req: &HttpRequest<AppState>) -> bool {

    req.session().get::<RawUser>("user").unwrap().is_none()
}