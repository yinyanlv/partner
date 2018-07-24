use actix::Arbiter;
use actix_web::{Error, Request};
use actix_web::pred::Predicate;
use actix_web::middleware::session::RequestSession;
use actix_redis::{Command, RespValue};
use cookie::{Cookie, CookieJar, Key};
use futures::Future;

use common::lazy_static::CONFIG;
use common::state::AppState;
use models::user::RawUser;

pub struct Unauthorized;

pub struct CheckLogin;

impl Predicate<AppState> for CheckLogin {

    fn check(&self, req: &Request, state: &AppState) -> bool {

        req.extensions_mut().insert(Unauthorized);

        false
    }
}

fn do_check(req: &Request, state: &AppState) -> bool {

    let cookie = req.headers().get("cookie");

    if cookie.is_none() {

        return false;
    }

    let cookie_str = cookie.unwrap().to_str().unwrap();

    let cookies = Cookie::parse(cookie_str);

    for cookie in cookies {

        if cookie.name() == &*CONFIG.cookie.key {

            let mut jar = CookieJar::new();
            jar.add_original(cookie.clone());

            if let Some(cookie) = jar.signed(&Key::from_master(&[0;32])).get(&*CONFIG.cookie.key) {

                let addr = state.redis_addr.clone();
                let result = Arbiter::spawn_fn(move || {
                    addr
                        .send(Command(resp_array!["GET", cookie.value()]))
                        .map_err(Error::from)
                        .then(move |res| {
                            match res {
                                Ok(val) => {
                                    println!("{:?}", val);
                                    true
                                },
                                Err(err) => {
                                    false
                                }
                            }
                        })
                }).wait();

                return true;
            }
        }
    }

    false
}
