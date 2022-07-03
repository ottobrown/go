use std::path::PathBuf;
use std::fs;
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

    events.move_to_root();

    return Ok(events);
}

fn build_event_tree(events: &mut EventTree, game: GameTree) {
    for n in game.nodes {
        for t in n.tokens {
            events.push(token_to_event(t));
        }
    }

    for v in game.variations {
        build_event_tree(events, v);

        events.move_to_parent();
    }
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
            color: _,
            action: Action::Move(x, y),
        } => Event::Move((x-1) as usize, (y-1) as usize),

        SgfToken::Add {
            color,
            coordinate: (x, y),
        } => Event::Place(color_to_stone(color), (x-1) as usize, (y-1) as usize),

        _ => Event::Noop,
    }
}
