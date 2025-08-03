use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub enum Player {
    PLAYER_1,
    PLAYER_2,
} 