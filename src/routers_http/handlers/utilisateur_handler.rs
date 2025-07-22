use std::fmt::format;

use actix_web::{get, post, web};
use sea_orm::{ActiveValue::Set, EntityTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;
use crate::utils::{api_response::{self, ApiResponse}, app_state::{ AppState}, jwt::Claims};
#[derive(Serialize,Deserialize)]
struct  UpdateUser{
    pseudo: String
}
#[get("")]
pub async fn user(
    app_state: web::Data<AppState>,
    data_claim: Claims
) -> Result<ApiResponse,ApiResponse> {
    let user_model = entity::utilisateur::Entity::find_by_id(data_claim.id)
        .one(&app_state.db).await
        .map_err(|error| ApiResponse::new(500, error.to_string()))?
        .ok_or(ApiResponse::new(404, "user not found".to_string()))?;

    Ok(api_response::ApiResponse::new(200, format!("{{'pseudo':{},'email':{} }}",user_model.pseudo,user_model.email)))
}

#[post("update")]
pub async fn update(
    app_state: web::Data<AppState>,
    user_data: web::Json<UpdateUser>,
    data_claim: Claims
) -> Result<ApiResponse,ApiResponse> {
    let mut user_model = entity::utilisateur::Entity::find_by_id(data_claim.id)
        .one(&app_state.db).await
        .map_err(|error| ApiResponse::new(500, error.to_string()))?
        .ok_or(ApiResponse::new(404, "user not found".to_string()))?
        .into_active_model();
    user_model.pseudo = Set(user_data.pseudo.clone());
    user_model.update(&app_state.db).await
    .map_err(|error| ApiResponse::new(500, error.to_string()))?;

    Ok(api_response::ApiResponse::new(200, "utilisateur".to_string()))
}