use actix_web::{ post, web };
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sha256::digest;

use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::encode_jwt;
use crate::utils::{api_response, app_state};

#[derive(Serialize,Deserialize)]
struct RegisterModel{
    pseudo:String,
    email:String,
    password:String
}

#[derive(Serialize,Deserialize)]
struct LoginModel {
    pseudo:String,
    password:String
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>
) -> Result<ApiResponse,ApiResponse>{
    let utilisateur = entity::utilisateur::ActiveModel{
        pseudo: Set(register_json.pseudo.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }.insert(&app_state.db).await
    .map_err(|error| ApiResponse::new(500,error.to_string()))?;

    Ok(api_response::ApiResponse::new(200, format!("{}",utilisateur.id)))
}
#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>
) -> Result<ApiResponse,ApiResponse>{

    let utilisateur = entity::utilisateur::Entity::find()
        .filter(
            Condition::all()
            .add(entity::utilisateur::Column::Pseudo.eq(&login_json.pseudo))
            .add(entity::utilisateur::Column::Password.eq(digest(&login_json.password)))
        ).one(&app_state.db).await
        .map_err(|error| ApiResponse::new(500,error.to_string()))?
        .ok_or(ApiResponse::new(404, "user not found".to_owned()))?;

    let key = encode_jwt(utilisateur.pseudo, utilisateur.id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    let body = json!({ "token": key }).to_string();
    Ok(api_response::ApiResponse::new(200, body))
}