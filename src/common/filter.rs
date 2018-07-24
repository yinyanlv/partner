use actix_web::Request;
use actix_web::pred::Predicate;
use actix_web::middleware::session::RequestSession;

use models::user::RawUser;

pub struct Unauthorized;

pub struct CheckLogin;

impl<S> Predicate<S> for CheckLogin {
    
    fn check(&self, req: &Request, state: &S) -> bool {

//        req.extensions_mut().insert(Unauthorized);
//
//        req.session().get::<RawUser>("user").unwrap().is_some()

        true
    }
}
