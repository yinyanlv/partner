use actix_web::http::{header, HttpTryFrom};
use actix_web::{App, HttpRequest, HttpResponse, Result};
use actix_web::middleware::{Middleware, Started, Response};
use actix_web::middleware::session::RequestSession;

use common::util::get_redis_key;
use common::state::AppState;

pub struct Remember;  

impl Middleware<AppState> for Remember {

    fn start(&self, req: &mut HttpRequest<AppState>) -> Result<Started> {
        
        println!("request ready");

        Ok(Started::Done)
    }

    fn response(&self, req: &mut HttpRequest<AppState>, mut res: HttpResponse) -> Result<Response> {

        let session = req.session();

        match session.get::<bool>("remember") {

            Ok(data) => {

                println!("{:?}", data);

                let redis_key = get_redis_key(req);

                if redis_key.is_some() {
                    println!("redis key: {}", redis_key.unwrap());
                }

                res.headers_mut().insert(
                    header::HeaderName::try_from("X-VERSION").unwrap(),
                    header::HeaderValue::from_static("0.2"));
            },
            Err(_) => {}
        }

        Ok(Response::Done(res))
    }
}
