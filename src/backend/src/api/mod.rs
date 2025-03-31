use actix_web::{Scope, web};

pub const API_SCOPE: &str = "api";

pub fn api_scope() -> Scope {
    web::scope(API_SCOPE)
}
