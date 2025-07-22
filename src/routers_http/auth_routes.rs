use actix_web::web;
use crate::routers_http::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
        .service(handlers::auth_handler::register)
        .service(handlers::auth_handler::login)
            
    );
}