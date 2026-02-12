use std::{error::Error, fmt::Display};

use actix_web::{http, middleware::DefaultHeaders, web, App, HttpServer};
use actix_cors::Cors;
use actix::Actor;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ Database, DatabaseConnection};

use crate::{routers_websocket::websocket::lobby::Lobby, utils::{app_state::AppState}};

mod utils;
mod routers_http;
mod routers_websocket;
mod list_jeu;

#[derive(Debug)]
struct MainError{
    message:String
}
impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Error: {}",self.message)
    }
}
impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[actix_web::main]
async fn main() -> Result<(),MainError> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv::dotenv().ok();
    env_logger::init();
    let port = (*utils::constants::PORT).clone();
    let address = (*utils::constants::ADDRESS).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();
    let db: DatabaseConnection = Database::connect(database_url).await.map_err(|err|MainError{message:err.to_string()})?;
    let room_server = Lobby::new().start();
    // let room_server = Lobby::new(db.clone()).start();
    
    // Migrator::up(&db,None)
    // Migrator::fresh(&db)
    // .await.map_err(|err|MainError{message:err.to_string()})?;

    HttpServer::new(move|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8090")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
                    .max_age(3600)
            )
            .wrap(DefaultHeaders::new().add(("Referrer-Policy", "no-referrer")))
            .app_data(web::Data::new(AppState { db: db.clone() } ))
            .app_data(web::Data::new(room_server.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(routers_http::home_routes::config)
            .configure(routers_http::auth_routes::config)
            .configure(routers_http::utilisateur_routes::config)
            .configure(routers_http::room_routes::config)

            .configure(routers_websocket::room_routes::config)
    })
    .bind((address, port))
    .map_err(|err|MainError{message:err.to_string()})?
    .run()
    .await
    .map_err(|err|MainError{message:err.to_string()})
}




