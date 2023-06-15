use crate::sgf::to_actions;
use crate::sgf::Action;
use crate::Board;
use crate::Stone;

/// Contains the Board and additional info about the game
/// that can be manipulated by actions in the ui
pub struct Game {
    pub board: Board,
    pub turn: Stone,

    pub tree: crate::SgfTree,
}
impl Game {
    pub fn do_action(&mut self, a: Action) {
        match a {
            Action::PlayBlack(x, y) => {
                self.board.attempt_set(x, y, Stone::Black);
                self.turn = Stone::White;
            }
            Action::PlayWhite(x, y) => {
                self.board.attempt_set(x, y, Stone::White);
                self.turn = Stone::Black;
            }

            _ => {}
        }
    }

    /// Starting from a blank board, performs all the actions up to this point in `self.tree`
    pub fn do_to_now(&mut self) {
        let (w, h) = self.board.size();
        self.board = Board::new(w, h);
        self.turn = Stone::Black;

        let all = self.tree.get_all_parent_text();

        for s in all.iter().rev() {
            // TODO: handle this error
            for a in to_actions(s).unwrap() {
                self.do_action(a);
            }
        }
    }
}

pub struct GameBuilder {
    pub size: (usize, usize),
}
impl GameBuilder {
    pub fn build(&self) -> Game {
        Game {
            board: Board::new(self.size.0, self.size.1),
            turn: Stone::Black,
            tree: crate::SgfTree::default(),
        }
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self { size: (19, 19) }
    }
}
