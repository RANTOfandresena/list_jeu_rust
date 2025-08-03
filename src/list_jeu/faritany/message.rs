use std::collections::HashMap;

use serde::{Serialize};

use crate::list_jeu::faritany::{player::Player, point::Point};

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum MessageServeurFaritany {
    #[serde(rename = "place-stone-faritany")]
    PlacementPoint {
        point: Point,
        player: String,
    },

    #[serde(rename = "assign-player-faritany")]
    AttributionJoueur {
        pseudo: String,
        localPlayer: String,
        currentPlayer: bool,
    },

    #[serde(rename = "points-faritany")]
    ListePointsFaritany {
        points : Vec<(String, Option<Player>)>,
    }
}