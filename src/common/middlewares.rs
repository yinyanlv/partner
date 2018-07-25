use actix::prelude::Arbiter;
use actix_web::{HttpRequest, HttpResponse, Result, Error};
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::middleware::{Middleware, Started, Response};
use actix_web::middleware::session::RequestSession;
use actix_redis::Command;
use cookie::{CookieJar, Key};
use chrono::Duration;
use futures::Future;

use common::state::AppState;
use common::lazy_static::CONFIG;
use models::user::RawUser;

pub struct Remember;  

impl Middleware<AppState> for Remember {

    fn response(&self, req: &HttpRequest<AppState>, mut res: HttpResponse) -> Result<Response> {

        let _req = &*req;
        
        match _req.session().get::<bool>("remember") {

            Ok(data) => {

                    if data.is_some() {

                        let remember = data.unwrap();

                        if remember {

                            let redis_key = get_redis_key(_req);

                            update_max_age(_req, &mut res);

                            if redis_key.is_some() {
                                let addr = _req.state().redis_addr.clone();

                                Arbiter::spawn_fn(move || {

                                    addr.send(Command(resp_array!["EXPIRE", &*redis_key.unwrap(), &*CONFIG.redis.ttl.to_string()]))
                                        .map_err(Error::from)
                                        .then(move |_res| {
                                            Ok(())
                                        })
                                });
                            }
                            
                        };
                                    
                    }
                },
            Err(_) => ()
        }

        Ok(Response::Done(res))
    }
}

fn get_redis_key(req: &HttpRequest<AppState>) -> Option<String> {

    let cookies = req.cookies().unwrap();

    for cookie in cookies.iter() {

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

fn update_max_age(req: &HttpRequest<AppState>, res: &mut HttpResponse) {

    let cookies = req.cookies().unwrap();
    let mut temp = None;

    for cookie in cookies.iter() {

        if cookie.name() == &*CONFIG.cookie.key {

            let mut c = cookie.clone();

            c.set_http_only(true);
            c.set_path("/".to_owned());
            c.set_max_age(Duration::seconds(CONFIG.cookie.max_age as i64));

            temp = Some(c);
        }
    }

    if temp.is_some() {
        res.headers_mut().append(header::SET_COOKIE, HeaderValue::from_str(&temp.unwrap().to_string()).unwrap());
    }
}

pub struct IsLoggedIn;
pub struct MarkLoginState;

impl Middleware<AppState> for MarkLoginState {

    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {

        let is_logged_in = req.session().get::<RawUser>("user").unwrap().is_some();

        if is_logged_in {
            req.extensions_mut().insert(IsLoggedIn);
        }

        Ok(Started::Done)
    }
}