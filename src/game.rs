use crate::Board;
use crate::Rules;
use crate::Stone;
use crate::rules::EndGame;

#[derive(Clone)]
#[allow(unused)]
pub enum Event {
    Pass,
    Resign(Stone),
    Move(usize, usize),

    Place(Stone, usize, usize),
}

/// <0 is kyu,
/// >0 is dan,
/// 0 is none
type Rank = i8;
 
pub fn display_rank(rank: Rank) -> String {
    if rank < 0 {
        return format!("{}k", rank.abs())
    }
    else if rank > 0 {
        return format!("{}d", rank)
    }
    else {
        return String::new()
    }
}

#[derive(Clone)]
pub struct Game {
    current_board: Board,
    history: Vec<Event>,

    turn: Stone,
    rules: Rules,
    end_game: Option<EndGame>,

    pub black_player: String,
    pub white_player: String,
    pub black_rank: Rank,
    pub white_rank: Rank,
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
            },
            Event::Pass => { self.turn = self.turn.swap() },

            Event::Resign(s) => { self.end_game = Some(EndGame::Resign(*s)) },
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

    pub black_player: String,
    pub white_player: String,
    pub black_rank: Rank,
    pub white_rank: Rank,
}
impl NewGameBuilder {
    pub fn build(&self) -> Game {
        Game {
            current_board: Board::blank(self.size.0, self.size.1),
            history: Vec::new(),

            turn: Stone::Black,
            rules: self.rules,
            end_game: None,

            black_player: self.black_player.clone(),
            white_player: self.white_player.clone(),
            black_rank: self.black_rank,
            white_rank: self.white_rank,
        }
    }
}

impl Default for NewGameBuilder {
    fn default() -> Self {
        Self {
            size: (19, 19),
            rules: Rules::CHINESE,

            black_player: String::from("Black"),
            white_player: String::from("White"),
            black_rank: 0,
            white_rank: 0,
        }
    }
}
