use actix::prelude::{Message, Recipient};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{list_jeu::faritany::point::Point, utils::jwt::Claims};

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message,Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Claims,
    pub type_jeu: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub user: Claims,
    pub room_id: Uuid,
}

#[derive(Message,Clone)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub user: Claims,
    pub msg: MessageClient,
    pub room_id: Uuid,
    pub type_jeu: String
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum MessageClient {
    #[serde(rename = "place-stone")]
    PlaceStone {
        point: Point,
    },
}

#[derive(Serialize)]
pub struct MessageServeur {
    pub type_m: String,
    pub point: Point,
    pub player: String,
}

#[derive(Serialize)]
pub struct MessageServeurFanorona {
    pub type_m: String,
    pub pseudo: String,
    pub localPlayer: String,
    pub currentPlayer: bool,
}
