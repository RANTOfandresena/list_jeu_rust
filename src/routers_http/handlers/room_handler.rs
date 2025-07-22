use actix_web::{ post, web};
use sea_orm::{ ActiveValue::Set};
use serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;
use uuid::Uuid;
use crate::utils::{api_response::{self, ApiResponse}, app_state::{ AppState}};
#[derive(Serialize,Deserialize,Clone)]
struct  CreateRoomModel{
    password: Option<String>,
    taille: String,
    reflexion: i32,
}
#[post("create")]
pub async fn create(
    app_state: web::Data<AppState>,
    room_data: web::Json<CreateRoomModel>
) -> Result<ApiResponse,ApiResponse> {

    let room = entity::faritany_room::ActiveModel {
        uuid: Set(Uuid::new_v4().to_string()),
        password: Set(room_data.password.clone()),
        taille: Set(room_data.taille.clone()),
        reflexion: Set(room_data.reflexion),
        ..Default::default()
    }.insert(&app_state.db).await
    .map_err(|error| ApiResponse::new(500,error.to_string()))?;

    // entity::faritany_room_utilisateur::ActiveModel {
    //     utilisateur_id: Set(user_model.id),
    //     room_id: Set(room.id),
    //     ..Default::default()
    // }.insert(&app_state.db).await
    // .map_err(|error| ApiResponse::new(500,error.to_string()))?;

    Ok(api_response::ApiResponse::new(200, format!("{{'uuid':'{}','password': '{}'}}",room.uuid,room.password.clone().unwrap_or_default())))
}