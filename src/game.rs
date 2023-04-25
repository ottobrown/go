use crate::Board;
use crate::Stone;

/// Contains the Board and additional info about the game
/// that can be manipulated by actions in the ui
pub struct Game {
    pub board: Board,
    pub turn: Stone,
}

pub struct GameBuilder {
    pub size: (usize, usize),
}
impl GameBuilder {
    pub fn build(&self) -> Game {
        Game {
            board: Board::new(self.size.0, self.size.1),
            turn: Stone::Black,
        }
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self { size: (19, 19) }
    }
}
