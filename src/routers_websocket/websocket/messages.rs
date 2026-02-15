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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageWebSocket {

    #[serde(rename = "user_joined")]
    UserJoined {
        user_id: i32,
        pseudo: String,
    },

    #[serde(rename = "user_disconnected")]
    UserDisconnected {
        user_id: i32,
        pseudo: String,
    },

    #[serde(rename = "chat")]
    Chat {
        user_id: i32,
        message: String,
    },
}
