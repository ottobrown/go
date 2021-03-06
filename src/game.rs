use crate::rules::EndGame;
use crate::tree::EventTree;
use crate::Board;
use crate::Rules;
use crate::Stone;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Marker {
    Empty,
    Triangle,
    Circle,
    Square,
    Cross,
    /// Contains the coordinates of the end point of the line
    Line(usize, usize),
    Arrow(usize, usize),
    Label(char),
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(unused)]
pub enum Event {
    Start,
    Pass,
    Resign(Stone),
    Move(usize, usize),

    Place(Stone, usize, usize),

    Mark(Marker, usize, usize),
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

    pub end_game: EndGame,
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

            end_game: EndGame::NotOver,
        }
    }
}

#[derive(Clone)]
pub struct Game {
    current_board: Board,
    history: EventTree,

    pub turn: Stone,
    initial_turn: Stone,
    rules: Rules,

    pub info: GameInfo,
}
impl Game {
    pub fn builder() -> NewGameBuilder {
        NewGameBuilder::default()
    }

    pub fn current_board(&self) -> Board {
        self.current_board.clone()
    }

    fn build_board_from_history(&mut self, history: &Vec<Event>) {
        let mut board = Board::blank(self.current_board.width(), self.current_board.height());
        let mut turn = self.initial_turn;

        for e in history {
            match e {
                Event::Place(s, x, y) => {
                    if board.play(*s, *x, *y, &self.rules) {
                        self.current_board.clear_markers();
                    }
                }
                Event::Move(x, y) => {
                    if board.play(turn, *x, *y, &self.rules) {
                        turn = !turn;
                        self.current_board.clear_markers();
                    }
                }
                Event::Pass => turn = !turn,
                Event::Mark(m, x, y) => {
                    board.set_marker(*m, *x, *y);
                }

                _ => {}
            }
        }

        self.current_board = board;
        self.turn = turn;
    }

    pub fn undo(&mut self) {
        self.pop_history();
        self.build_board_from_history(&self.history.get_history());
    }

    fn pop_history(&mut self) -> Option<Event> {
        self.history.pop()
    }

    pub fn move_back(&mut self) {
        self.history.move_to_parent();
        self.build_board_from_history(&self.history.get_history());
    }

    pub fn move_forward(&mut self) {
        self.history.move_to_first_child();
        self.build_board_from_history(&self.history.get_history());
    }

    pub fn move_up(&mut self) {
        self.history.move_to_last_sibling();
        self.build_board_from_history(&self.history.get_history());
    }

    pub fn move_down(&mut self) {
        self.history.move_to_next_sibling();
        self.build_board_from_history(&self.history.get_history());
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
                    // Remove event if it was illegal
                    self.current_board.clear_markers();
                    self.pop_history();
                }
            }
            Event::Move(x, y) => {
                if self.current_board.play(self.turn, *x, *y, &self.rules) {
                    self.turn = !self.turn;
                    self.current_board.clear_markers();
                } else {
                    // Remove event if it was illegal
                    self.pop_history();
                }
            }
            Event::Pass => self.turn = !self.turn,

            Event::Resign(s) => self.info.end_game = EndGame::Resign(*s),

            Event::Mark(m, x, y) => {
                if !self.current_board.set_marker(*m, *x, *y) {
                    // Remove event if it did nothing
                    self.pop_history();
                }
            }

            _ => {}
        };
    }

    pub fn size(&self) -> (usize, usize) {
        (self.current_board.width(), self.current_board.height())
    }

    pub fn ended(&self) -> bool {
        let mut path = self.history.get_history();

        if path.pop() == Some(Event::Pass) && path.pop() == Some(Event::Pass) {
            return true;
        }

        if let Some(Event::Resign(_)) = path.last() {
            return true;
        }

        return false;
    }
}

/// Used to build a new, blank game
pub struct NewGameBuilder {
    pub size: (usize, usize),
    pub rules: Rules,

    pub info: GameInfo,
}
impl NewGameBuilder {
    pub fn build(&self) -> Game {
        Game {
            initial_turn: Stone::Black,
            current_board: Board::blank(self.size.0, self.size.1),
            history: EventTree::blank(),

            turn: Stone::Black,
            rules: self.rules,

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
        }
    }
}
