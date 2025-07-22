use actix_web::{ web};

use crate::{ routers_websocket::handlers};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/room_ws")
            .service(handlers::start_connection::start_connection)
    );
}