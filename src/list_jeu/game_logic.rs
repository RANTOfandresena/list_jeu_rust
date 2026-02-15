use crate::routers_websocket::websocket::messages::MessageClient;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};


#[derive(Eq, PartialEq)]
pub struct VecKey(pub Vec<i32>);

impl Hash for VecKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in &self.0 {
            v.hash(state);
        }
    }
}
pub trait GameLogic {
    // action du client spécifique à un jeu
    fn handle_client_message(&mut self, message_content: &MessageClient, user_id: &i32) -> HashMap<VecKey, String>;

    // connexion d'un utilisateur à un jeu
    fn handle_connect(&mut self, user_id: i32, user_pseudo: String) -> HashMap<VecKey, String>;

    // deconnexion d'un client spécifique à un jeu
    fn handle_deconnect(&mut self, user_id: i32, user_pseudo: String) -> HashMap<VecKey, String>;
}