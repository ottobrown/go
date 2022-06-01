use crate::rules::EndGame;
use crate::Board;
use crate::Rules;
use crate::Stone;

#[derive(Clone)]
#[allow(unused)]
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

    pub turn: Stone,
    rules: Rules,
    pub end_game: Option<EndGame>,
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
            Event::Pass => self.turn = self.turn.swap(),

            Event::Resign(s) => self.end_game = Some(EndGame::Resign(*s)),
        };
    }

    pub fn size(&self) -> (usize, usize) {
        (self.current_board.width(), self.current_board.height())
    }
}

/// Used to build a new, blank game
pub struct NewGameBuilder {
    pub size: (usize, usize),
    pub rules: Rules,
}
impl NewGameBuilder {
    pub fn build(&self) -> Game {
        Game {
            current_board: Board::blank(self.size.0, self.size.1),
            history: Vec::new(),
            turn: Stone::Black,
            rules: self.rules,

            end_game: None,
        }
    }
}

impl Default for NewGameBuilder {
    fn default() -> Self {
        Self {
            size: (19, 19),
            rules: Rules::JAPANESE,
        }
    }
}
