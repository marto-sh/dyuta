use uuid::Uuid;

use crate::player::entity::Player;

/// Represents a particular game session that occured (e.g. Jessica and Andrew played Checkers on January 2nd)
///
/// Not to be confused with a game (e.g. Checkers)
pub struct GameSession {
    /// Unique identifier for this game session
    ///
    /// Used as a seed to generate randomness
    pub id: Uuid,

    /// List of players playing in this game session
    pub players: Vec<Player>,
}
