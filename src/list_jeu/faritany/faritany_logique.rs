use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::list_jeu::faritany::grid::Grid;
use crate::list_jeu::faritany::message::MessageServeurFaritany;
use crate::list_jeu::faritany::player::Player;
use crate::list_jeu::game_logic::GameLogic;
use crate::routers_websocket::websocket::messages::{MessageClient};

#[derive(Debug, Clone)]
pub struct FaritanyLogique {
    pub grid: Grid,
    pub joueur_courant: Player, 
    pub joueurs: HashMap<i32, Player>,
    pub tour_commence_at: Option<DateTime<Utc>>,
    pub duree_tour_secondes: i64,
}


impl FaritanyLogique {
    pub fn new(size: i32,reflexion: i64) -> Self {
        Self {
            grid: Grid::new(size),
            joueur_courant: Player::PLAYER_1,
            joueurs: HashMap::new(),
            tour_commence_at: None, 
            duree_tour_secondes: reflexion,
        }
    }
    pub fn get_role(&self, user_id: &i32) -> Option<&Player> {
        self.joueurs.get(user_id)
    }
    fn passer_tour(&mut self) {
        self.joueur_courant = match self.joueur_courant {
            Player::PLAYER_1 => Player::PLAYER_2,
            Player::PLAYER_2 => Player::PLAYER_1,
        };
    }

    fn verifier_tour(&mut self, joueur: Player) -> bool {
        if let Some(debut) = self.tour_commence_at {
            let maintenant = Utc::now();
            let ecoule = maintenant.signed_duration_since(debut);
            let secondes_ecoulees = ecoule.num_seconds();

            if (secondes_ecoulees / self.duree_tour_secondes) % 2 == 1 && self.joueur_courant != joueur {
                self.passer_tour();
                return true;
            } else if (secondes_ecoulees / self.duree_tour_secondes) % 2 == 1 && self.joueur_courant == joueur {
                return false;
            }
        } else {
            self.tour_commence_at = Some(Utc::now());
        }
        true
    }
}
impl GameLogic for FaritanyLogique {
    fn handle_client_message(&mut self, message_content: &MessageClient, user_id: &i32) -> Option<String> {
        let role = match self.get_role(user_id) {
            Some(r) => *r,
            None => return None,
        };

        if !self.verifier_tour(role) {
            return None;
        }

        let MessageClient::PlaceStone { point, .. } = message_content;

        if self.grid.is_cell_empty(point) && self.joueur_courant == role {
            self.grid.place_stone(*point, self.joueur_courant);

            let message_server = serde_json::to_string(&MessageServeurFaritany::PlacementPoint {
                point: *point,
                player: format!("{:?}", self.joueur_courant),
            }).unwrap();

            self.passer_tour();
            self.tour_commence_at = Some(Utc::now());

            return Some(message_server);
        }

        None
    }

    fn handle_connect(&mut self, user_id: i32, user_pseudo: String) -> Option<String> {
        if self.joueurs.contains_key(&user_id) {
            let points_vec: Vec<(String, Option<Player>)> = self.grid
                .get_Cell()
                .into_iter()
                .collect();

            let message = MessageServeurFaritany::ListePointsFaritany { points: points_vec };
            return serde_json::to_string(&message).ok();
        }
        let role = match self.joueurs.len() {
            0 => Player::PLAYER_1,
            1 => Player::PLAYER_2,
            _ => {
                println!("Déjà 2 joueurs ont été assignés.");
                return None;
            }
        };

        self.joueurs.insert(user_id, role);

        let message = MessageServeurFaritany::AttributionJoueur {
            pseudo: user_pseudo,
            localPlayer: format!("{:?}", role),
            currentPlayer: self.joueur_courant == role,
        };

        serde_json::to_string(&message).ok()
    }

    fn handle_deconnect(&mut self, user_id: i32, _user_pseudo: String) -> Option<String> {
        self.joueurs.remove(&user_id);
        Some(format!("{} s'est déconnecté.", user_id))
    }
}