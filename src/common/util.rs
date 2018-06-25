use crypto::md5::Md5;
use crypto::digest::Digest;
use rand::{thread_rng, Rng};
use actix_web::HttpRequest;
use actix_web::middleware::session::RequestSession;
use cookie::{CookieJar, Key};

use common::lazy_static::CONFIG;
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

pub fn get_redis_key(req: &mut HttpRequest<AppState>) -> Option<String> {

    let cookies = req.cookies().unwrap();

    for cookie in cookies {

        if cookie.name() == &*CONFIG.cookie.key {

            let mut jar = CookieJar::new();
            jar.add_original(cookie.clone());

            if let Some(cookie) = jar.signed(&Key::from_master(&[0;32])).get(&*CONFIG.cookie.key) { 

                return Some(cookie.value().to_owned());
            }
        }
    }

    None
}