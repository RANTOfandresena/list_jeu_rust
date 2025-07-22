use std::collections::HashMap;

use crate::list_jeu::faritany::grid::Grid;
use crate::list_jeu::faritany::player::Player;
use crate::list_jeu::game_logic::GameLogic;
use crate::routers_websocket::websocket::messages::{MessageClient, MessageServeur, MessageServeurFanorona};

#[derive(Debug, Clone)]
pub struct FaritanyLogique {
    pub grid: Grid,
    pub joueur_courant: Player, 
    pub joueurs: HashMap<i32, Player>,
}


impl FaritanyLogique {
    pub fn new(size: i32) -> Self {
        Self {
            grid: Grid::new(size),
            joueur_courant: Player::PLAYER_1,
            joueurs: HashMap::new(),
        }
    }
    pub fn atribuer_role(&mut self, joueur_id: i32, pseudo: String) -> Option<String> {
        if self.joueurs.contains_key(&joueur_id) {
            return None;
        }
        let role = match self.joueurs.len() {
            0 => Player::PLAYER_1,
            1 => Player::PLAYER_2,
            _ => {
                println!("Déjà 2 joueurs ont été assignés.");
                return None;
            }
        };

        self.joueurs.insert(joueur_id, role);

        let message = MessageServeurFanorona {
            pseudo,
            localPlayer: format!("{:?}", Some(role)),
            currentPlayer: self.joueur_courant == role,
        };

        serde_json::to_string(&message).ok()
    }

    pub fn get_role(&self, joueur_id: &i32) -> Option<&Player> {
        self.joueurs.get(joueur_id)
    }

    pub fn jouer_coup(&mut self, coup: &MessageClient, joueur_id: &i32) -> Option<String> {

        let MessageClient::PlaceStone { point, .. } = coup;
        if let Some(role) = self.get_role(joueur_id) {
            if self.grid.is_cell_empty(point) && self.joueur_courant == *role {
                self.grid.place_stone(*point, self.joueur_courant);

                let message_server = serde_json::to_string(&MessageServeur {
                        type_: "place-stone".to_string(),
                        point: *point,
                        player: format!("{:?}", self.joueur_courant),
                    }).unwrap();
                
                self.joueur_courant = if self.joueur_courant == Player::PLAYER_1 {
                    Player::PLAYER_2
                } else {
                    Player::PLAYER_1
                };

                return Some(message_server);
            }
        }

        None
    }
}
impl GameLogic for FaritanyLogique {
    fn handle_client_message(&mut self, message_content: &str, user_id: &i32) -> Option<String> {
        todo!()
    }

    fn handle_connect(&mut self, user_id: i32, user_pseudo: String) -> Option<String> {
        todo!()
    }

    fn handle_deconnect(&mut self, user_id: i32, user_pseudo: String) -> Option<String> {
        todo!()
    }
}