use std::path::PathBuf;

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

#[derive(Clone, Debug)]
pub enum Event {
    /// The event at the root of the [GameTree].
    /// Does nothing.
    Start,
    /// A player passes their turn.
    Pass,
    /// A player resigns the game.
    Resign(Stone),
    /// Place a stone from the palyer whose turn it is.
    Move(usize, usize),
    /// Place a stone of the given color.
    Place(Stone, usize, usize),
    /// Used to mark up the board.
    Mark(Marker, usize, usize),
    /// Comment attached to a [Event::Group]
    Comment(String),
    /// Events grouped together so they can be handled at the same time.
    /// e.g. a bunch of [Event::Mark]s, [Event::Place]s
    Group(Vec<Event>),
}
impl Event {
    pub fn add_to_group(&mut self, e: Event) {
        if let Event::Group(ref v) = e {
            let mut vec = v.clone();

            vec.push(e.clone());

            *self = Event::Group(vec);
        } else {
            let vec = vec![self.clone(), e];

            *self = Event::Group(vec);
        }
    }

    pub fn comment(&self) -> Option<String> {
        match self {
            Event::Group(v) => {
                for i in v {
                    if let Event::Comment(s) = i {
                        return Some(s.clone());
                    }
                }

                None
            },

            _ => None,
        }
    }

    pub fn add_comment(&mut self, comment: String) {
        if let Event::Group(v) = self {
            for i in 0..v.len() {
                if let Event::Comment(_) = v[i] {
                    v.remove(i);
                }
            }
        }

        self.add_to_group(Event::Comment(comment))
    }
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

    pub sgf_path: Option<PathBuf>,
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

            sgf_path: None,
        }
    }
}

#[derive(Clone)]
pub struct Game {
    current_board: Board,
    initial_board: Board,
    initial_turn: Stone,
    pub history: EventTree,

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

        fn handle(e: &Event, board: &mut Board, turn: &mut Stone, rules: &Rules) {
            match e {
                Event::Place(s, x, y) => {
                    if board.play(*s, *x, *y, rules) {
                        board.clear_markers();
                    }
                }
                Event::Move(x, y) => {
                    if board.play(*turn, *x, *y, rules) {
                        *turn = turn.swap();
                        board.clear_markers();
                    }
                }
                Event::Pass => *turn = turn.swap(),
                Event::Mark(m, x, y) => {
                    board.set_marker(*m, *x, *y);
                }

                Event::Group(v) => {
                    for i in v {
                        handle(i, board, turn, rules);
                    }
                }

                _ => {}
            }
        }

        for e in history {
            handle(e, &mut board, &mut turn, &self.rules);
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
        self.history.push(e.clone());

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
                    self.turn = self.turn.swap();
                    self.current_board.clear_markers();
                } else {
                    // Remove event if it was illegal
                    self.pop_history();
                }
            }
            Event::Pass => self.turn = self.turn.swap(),

            Event::Resign(s) => self.end_game = Some(EndGame::Resign(*s)),

            Event::Mark(m, x, y) => {
                self.pop_history();

                let current_event = self.history.get_current_event_mut();
                if self.current_board.set_marker(*m, *x, *y) {
                    current_event.add_to_group(e.clone());
                }
            }

            Event::Comment(s) => {
                let current_event = self.history.get_current_event_mut();
                current_event.add_comment(s.clone());
            }

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
    pub tree: Option<EventTree>,
}
impl NewGameBuilder {
    pub fn build(&self) -> Game {
        Game {
            initial_board: Board::blank(self.size.0, self.size.1),
            initial_turn: Stone::Black,
            current_board: Board::blank(self.size.0, self.size.1),
            history: match &self.tree {
                Some(t) => t.clone(),
                None => EventTree::blank(),
            },

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
            tree: None,
        }
    }
}
