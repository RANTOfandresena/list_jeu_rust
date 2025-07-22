use actix_web::web;
use crate::routers_http::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(handlers::home_handler::hello)
            .service(handlers::home_handler::home)
    );
}