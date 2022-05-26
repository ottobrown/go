use crate::Board;
use crate::Rules;
use crate::Stone;

#[derive(Clone)]
pub enum Event {
    Pass,
    Resign(Stone),
    Move(usize, usize),

    Place(Stone, usize, usize),
}

#[derive(Clone)]
pub struct Game {
    current_board: Board,
    history: Vec<Event>,

    turn: Stone,
    rules: Rules,
}
impl Game {
    pub fn builder() -> NewGameBuilder {
        NewGameBuilder::default()
    }

    pub fn current_board(&self) -> Board {
        self.current_board.clone()
    }

    pub fn handle_event(&mut self, e: &Event) {
        self.history.push(e.clone());

        match e {
            Event::Place(s, x, y) => self.current_board.set(*s, *x, *y),
            Event::Move(x, y) => {
                if self.current_board.play(self.turn, *x, *y, &self.rules) {
                    self.turn = self.turn.swap();
                }
            }

            _ => unimplemented!(),
        };
    }

    pub fn size(&self) -> (usize, usize) {
        (self.current_board.width(), self.current_board.height())
    }
}

/// Used to build a new, blank game
pub struct NewGameBuilder {
    pub w: usize,
    pub h: usize,
    pub rules: Rules,
}
impl NewGameBuilder {
    pub fn build(&self) -> Game {
        Game {
            current_board: Board::blank(self.size.0, self.size.1),
            history: Vec::new(),
            turn: Stone::Black,
            rules: self.rules,
        }
    }
}

impl Default for NewGameBuilder {
    fn default() -> Self {
        Self {
            w: 19,
            h: 19,
            rules: Rules::NEW_ZEALAND,
        }
    }
}
