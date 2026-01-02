use std::sync::Arc;

/// Unique game identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameId(pub Arc<str>);

impl GameId {
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Represents a game with its metadata
pub struct Game {
    /// Unique identifier for this game
    pub id: GameId,

    /// Name of the game, for display purposes
    pub name: String,

    /// Description of the same, for display purposes
    pub description: String,

    /// Rules of the game, both for display purposes and for robots
    pub rules: String,
}
