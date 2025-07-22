use crate::routers_websocket::websocket::messages::MessageClient;

pub trait GameLogic {
    // action du client spécifique à un jeu
    fn handle_client_message(&mut self, message_content: &MessageClient, user_id: &i32) -> Option<String>;

    // connexion d'un utilisateur à un jeu
    fn handle_connect(&mut self, user_id: i32, user_pseudo: String) -> Option<String>;

    // deconnexion d'un client spécifique à un jeu
    fn handle_deconnect(&mut self, user_id: i32, user_pseudo: String) -> Option<String>;
}