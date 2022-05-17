use crate::Board;
use crate::Stone;

#[derive(Clone)]
pub enum Event {
    Pass(Stone),
    Resign(Stone),
    Place(Stone, usize, usize),
    Edit(Board),
}

pub struct Game {
    current_board: Board,
    history: Vec<Event>,
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
        }
    }
}

impl Default for NewGameBuilder {
    fn default() -> Self {
        Self { w: 19, h: 19 }
    }
}
