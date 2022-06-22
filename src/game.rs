use std::path::PathBuf;

use crate::rules::EndGame;
use crate::tree::EventTree;
use crate::Board;
use crate::Rules;
use crate::Stone;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Start,
    Pass,
    Resign(Stone),
    Move(usize, usize),

    Place(Stone, usize, usize),
}

/// <0 is kyu,
/// >0 is dan,
/// 0 is none
#[derive(Clone, Copy)]
pub struct Rank(pub i8);
#[allow(dead_code)]
impl Rank {
    pub fn none() -> Self {
        Self(0)
    }

    pub fn kyu(k: u8) -> Self {
        Self(-(k as i8))
    }

    pub fn dan(d: u8) -> Self {
        Self(d as i8)
    }

    pub fn display(&self) -> String {
        if self.0 < 0 {
            return format!("{}k", self.0.abs());
        }

        if self.0 > 0 {
            return format!("{}d", self.0);
        }

        return String::new();
    }
}

#[derive(Clone)]
pub struct GameInfo {
    pub name: String,
    pub event: String,
    pub comment: String,

    pub black_player: String,
    pub white_player: String,
    pub black_rank: Rank,
    pub white_rank: Rank,
}
impl Default for GameInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            event: String::new(),
            comment: String::new(),

            black_player: String::from("Black"),
            white_player: String::from("White"),
            black_rank: Rank::none(),
            white_rank: Rank::none(),
        }
    }
}

#[derive(Clone)]
pub struct Game {
    current_board: Board,
    initial_board: Board,
    initial_turn: Stone,
    history: EventTree,

    pub turn: Stone,
    rules: Rules,

    pub info: GameInfo,
    pub end_game: Option<EndGame>,
}
impl Game {
    pub fn builder() -> NewGameBuilder {
        NewGameBuilder::default()
    }

    pub fn current_board(&self) -> Board {
        self.current_board.clone()
    }

    fn build_board_from_history(&mut self, history: &Vec<Event>) {
        let mut board = self.initial_board.clone();
        let mut turn = self.initial_turn;

        for e in history {
            match e {
                Event::Place(s, x, y) => {
                    board.play(*s, *x, *y, &self.rules);
                }
                Event::Move(x, y) => {
                    if board.play(turn, *x, *y, &self.rules) {
                        turn = turn.swap();
                    }
                }
                Event::Pass => turn = turn.swap(),

                _ => {}
            }
        }

        self.current_board = board;
        self.turn = turn;
    }

    pub fn undo(&mut self) {
        self.pop_history();
        self.build_board_from_history(&self.history.get_path());
    }

    fn pop_history(&mut self) -> Option<Event> {
        self.history.pop()
    }

    pub fn move_back(&mut self) {
        self.history.move_to_parent();
        self.build_board_from_history(&self.history.get_path());
    }

    pub fn move_forward(&mut self) {
        self.history.move_to_first_child();
        self.build_board_from_history(&self.history.get_path());
    }

    pub fn move_up(&mut self) {
        self.history.move_to_last_sibling();
        self.build_board_from_history(&self.history.get_path());
    }

    pub fn move_down(&mut self) {
        self.history.move_to_next_sibling();
        self.build_board_from_history(&self.history.get_path());
    }

    pub fn black_prisoners(&self) -> u32 {
        self.current_board.black_prisoners
    }

    pub fn white_prisoners(&self) -> u32 {
        self.current_board.white_prisoners
    }

    pub fn handle_event(&mut self, e: &Event) {
        self.history.push(*e);

        match e {
            Event::Place(s, x, y) => {
                if !self.current_board.play(*s, *x, *y, &self.rules) {
                    // Remove event if it waws illegal
                    self.pop_history();
                }
            }
            Event::Move(x, y) => {
                if self.current_board.play(self.turn, *x, *y, &self.rules) {
                    self.turn = self.turn.swap();
                } else {
                    // Remove event if it waws illegal
                    self.pop_history();
                }
            }
            Event::Pass => self.turn = self.turn.swap(),

            Event::Resign(s) => self.end_game = Some(EndGame::Resign(*s)),

            _ => {}
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

    pub info: GameInfo,

    pub sgf_path: Option<PathBuf>,
}
impl NewGameBuilder {
    pub fn build(&self) -> Game {
        Game {
            initial_board: Board::blank(self.size.0, self.size.1),
            initial_turn: Stone::Black,
            current_board: Board::blank(self.size.0, self.size.1),
            history: EventTree::blank(),

            turn: Stone::Black,
            rules: self.rules,
            end_game: None,

            info: self.info.clone(),
        }
    }
}

impl Default for NewGameBuilder {
    fn default() -> Self {
        Self {
            size: (19, 19),
            rules: Rules::CHINESE,

            info: GameInfo::default(),
            sgf_path: None,
        }
    }
}
