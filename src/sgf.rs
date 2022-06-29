use std::path::PathBuf;
use std::{io, fs};
use std::error::Error;

use crate::Event;
use crate::Stone;
use crate::EventTree;

use sgf_parser::{
    GameTree,
    SgfToken,
    Action,
    Color,
};

use rfd::FileDialog;

/// Open a file dialog for a .sgf file
pub fn open_sgf() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Smart Game Format", &["sgf"])
        .pick_file()
}


pub fn parse_tree(path: PathBuf) -> Result<EventTree, Box<dyn Error>> {
    let string = fs::read_to_string(path)?;

    let game = sgf_parser::parse(&string)?;
    let mut events = EventTree::blank();

    build_event_tree(&mut events, game);

    return Ok(events);
}

fn build_event_tree(events: &mut EventTree, game: GameTree) {
    todo!()
}

fn color_to_stone(c: Color) -> Stone {
    match c {
        Color::Black => Stone::Black,
        Color::White => Stone::White,
    }
}

fn token_to_event(token: SgfToken) -> Event {
    match token {
        SgfToken::Move {
            color,
            action: Action::Move(x, y),
        } => Event::Move((x-1) as usize, (y-1) as usize),

        SgfToken::Add {
            color,
            coordinate: (x, y),
        } => Event::Place(color_to_stone(color), (x-1) as usize, (y-1) as usize),

        _ => Event::Noop,
    }
}
