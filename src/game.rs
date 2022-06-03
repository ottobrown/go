use crate::rules::EndGame;
use crate::Board;
use crate::Rules;
use crate::Stone;

#[derive(Clone, Copy, Debug)]
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
    initial_board: Board,
    initial_turn: Stone,
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

    fn build_board_from_history(&mut self) {
        let mut board = self.initial_board.clone();
        let mut turn = self.initial_turn;

        for e in &self.history {
            match e {
                Event::Place(s, x, y) => board.set(*s, *x, *y),
                Event::Move(x, y) => {
                    if board.play(turn, *x, *y, &self.rules) {
                        turn = turn.swap();
                    }
                },
                Event::Pass => turn = turn.swap(),

                _ => {}
            }
        }

        self.current_board = board;
        self.turn = turn;

    }

    pub fn undo(&mut self) {
        self.pop_history();
        self.build_board_from_history();
    }

    pub fn pop_history(&mut self) -> Option<Event> {
        self.history.pop()
    }

    pub fn handle_event(&mut self, e: &Event) {
        self.history.push(e.clone());

        match e {
            Event::Place(s, x, y) => self.current_board.set(*s, *x, *y),
            Event::Move(x, y) => {
                if self.current_board.play(self.turn, *x, *y, &self.rules) {
                    self.turn = self.turn.swap();
                }
            },
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
            initial_board: Board::blank(self.size.0, self.size.1),
            initial_turn: Stone::Black,
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
