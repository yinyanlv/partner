use actix_web::Request;
use actix_web::pred::Predicate;

use common::state::AppState;
use common::middlewares::IsLoggedIn;

pub struct Unauthorized;

pub struct CheckLogin;

impl Predicate<AppState> for CheckLogin {

    fn check(&self, req: &Request, _state: &AppState) -> bool {

        let is_logged_in = req.extensions().get::<IsLoggedIn>().is_some();

        if !is_logged_in {
            req.extensions_mut().insert(Unauthorized);
        }

        is_logged_in
    }
}
