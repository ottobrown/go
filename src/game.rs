use crate::Board;
use crate::Stone;

#[derive(Clone)]
pub enum Event {
    Pass,
    Resign(Stone),
    Move(usize, usize),

    Place(Stone, usize, usize),
}

pub struct Game {
    current_board: Board,
    history: Vec<Event>,

    turn: Stone,
}
impl Game {
    pub fn builder() -> NewGameBuilder {
        NewGameBuilder::default()
    }

    pub fn current_board(&self) -> Board {
        self.current_board.clone()
    }

    fn swap_turn(&mut self) {
        self.turn = match self.turn {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,

            // this should never happen
            _ => Stone::Empty,
        };
    }

    pub fn handle_event(&mut self, e: &Event) {
        self.history.push(e.clone());

        match e {
            Event::Place(s, x, y) => self.current_board.set(*s, *x, *y),
            Event::Move(x, y) => {
                if self.current_board.play(self.turn, *x, *y) {
                    self.swap_turn()
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
}
impl NewGameBuilder {
    pub fn build(&self) -> Game {
        Game {
            current_board: Board::blank(self.w, self.h),
            history: Vec::new(),
            turn: Stone::Black,
        }
    }
}

impl Default for NewGameBuilder {
    fn default() -> Self {
        Self { w: 19, h: 19 }
    }
}
