use std::sync::Arc;
use uuid::Uuid;

pub enum PlayerKind {
    Human,
    Robot,
}

/// A Player is an entity (e.g. a Human, a Bot) that can take actions in a Game
pub struct Player {
    /// Unique identifier for this player
    pub id: Uuid,

    /// Kind of player
    pub kind: PlayerKind,

    /// Name for this player.
    ///
    /// It does not have to be unique (e.g. John Doe)
    pub name: Arc<str>,
}
