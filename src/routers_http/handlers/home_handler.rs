use actix_web::{get, web, Responder};
use sea_orm::{ ConnectionTrait, Statement};

use crate::utils::{api_response::{self, ApiResponse}, app_state::AppState};

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(
        200,
        format!("Welcome to the home page! {name}"),
    )
}

#[get("/home")]
async fn home(
    app_state: web::Data<AppState>
) -> Result<ApiResponse,ApiResponse> {
    let res = app_state.db
        .query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "select * from utilisateur;")).await
    .map_err(|err| ApiResponse::new(500,err.to_string()))?;
    Ok(api_response::ApiResponse::new(
        200,
        "Welcome to the home page!".to_string(),
    ))
}