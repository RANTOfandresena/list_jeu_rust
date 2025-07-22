
use actix_web::{middleware::from_fn, web};
use crate::routers_http::{handlers, middlewares};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/utilisateur")
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            .service(handlers::utilisateur_handler::user)
            .service(handlers::utilisateur_handler::update)
    );
}