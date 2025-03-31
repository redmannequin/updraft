use actix_web::{Scope, dev::HttpServiceFactory, web};
use home::home;

mod component;
mod home;

pub const APP_SCOPE: &str = "/";

pub fn app_scope() -> impl HttpServiceFactory + 'static {
    Scope::new(APP_SCOPE).service(web::resource("").get(home))
}
