use std::str;
use actix::Arbiter;
use actix_web::{Error, Request, error::CookieParseError};
use actix_web::pred::Predicate;
use actix_web::middleware::session::RequestSession;
use actix_redis::{Command, RespValue};
use cookie::{Cookie, CookieJar, Key};
use futures::Future;
use futures::future::{err as FutErr, ok as FutOk, Either};

use common::lazy_static::CONFIG;
use common::state::AppState;
use models::user::RawUser;

pub struct Unauthorized;

pub struct CheckLogin;

impl Predicate<AppState> for CheckLogin {

    fn check(&self, req: &Request, state: &AppState) -> bool {

        req.extensions_mut().insert(Unauthorized);

        let is_authed = do_check(req, state).then(|res| {
            println!("{}", 11111);
            match res {
                Ok(val) => FutOk::<bool, bool>(true),
                Err(_) => FutErr::<bool, bool>(false)
            }
        });

        is_authed.wait().unwrap()
    }
}

fn do_check(req: &Request, state: &AppState) -> Box<Future<Item = Option<bool>, Error = Error> >{

    let cookies = get_cookies(&req).unwrap();

    for cookie in cookies.iter() {

        if cookie.name() == &*CONFIG.cookie.key {

            let mut jar = CookieJar::new();
            jar.add_original(cookie.clone());

            if let Some(cookie) = jar.signed(&Key::from_master(&[0;32])).get(&*CONFIG.cookie.key) {

                let addr = state.redis_addr.clone();

                return Box::new(
                    addr
                        .send(Command(resp_array!["GET", cookie.value()]))
                        .map_err(Error::from)
                        .and_then(move |res| {
                            match res {
                                Ok(val) => {
                                    println!("{:?}", val);
                                    return Ok(Some(true));
                                },
                                Err(err) => {
                                    return Ok(None);
                                }
                            }

                            Ok(None)
                        })
                );
            } else {
                return Box::new(FutOk(None));
            }
        }
    }

    Box::new(FutOk(None))
}

fn get_cookies(req: &Request) -> Result<Vec<Cookie<'static>>, CookieParseError> {

    let mut cookies = Vec::new();

    for item in req.headers().get_all("cookie") {

        let s = str::from_utf8(item.as_bytes()).map_err(CookieParseError::from)?;

        for cookie_str in s.split(';').map(|s| s.trim()) {
            if !cookie_str.is_empty() {
                cookies.push(Cookie::parse_encoded(cookie_str)?.into_owned());
            }
        }
    }

    Ok(cookies)
}
