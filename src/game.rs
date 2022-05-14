use crate::Board;

pub enum Event {
    Pass,
    Resign,
    Place(u8, u8),
    Edit(Board),
}

pub struct Game {
    current_board: Board,
    history: Vec<Event>,
}
