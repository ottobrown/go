use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::game::{GameInfo, Marker, NewGameBuilder};
use crate::Event;
use crate::EventTree;
use crate::Stone;

use sgf_parser::{Action, Color, GameTree, SgfToken};

use rfd::FileDialog;

/// Open a file dialog for a .sgf file
pub fn open_sgf() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Smart Game Format", &["sgf"])
        .pick_file()
}

pub fn parse_sgf(path: PathBuf, builder: &mut NewGameBuilder) -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string(path)?;

    let game = sgf_parser::parse(&string)?;
    let mut events = EventTree::blank();

    build_event_tree(&mut events, &mut builder.info, &mut builder.size, game);

    events.move_to_root();

    builder.tree = Some(events);

    return Ok(());
}

fn build_event_tree(
    events: &mut EventTree,
    info: &mut GameInfo,
    size: &mut (usize, usize),
    game: GameTree,
) {
    for n in game.nodes {
        if n.tokens.len() == 1 {
            let t = &n.tokens[0];

            token_to_info(t, info, size);

            if let Some(e) = token_to_event(t) {
                events.push(e);
            }

            continue;
        }

        let mut vec = Vec::new();

        for t in n.tokens {
            token_to_info(&t, info, size);

            if let Some(e) = token_to_event(&t) {
                vec.push(e);
            }
        }

        if !vec.is_empty() {
            events.push(Event::Group(vec.clone()));
        }
    }

    for v in game.variations {
        build_event_tree(events, info, size, v);

        events.move_to_parent();
    }
}

fn color_to_stone(c: Color) -> Stone {
    match c {
        Color::Black => Stone::Black,
        Color::White => Stone::White,
    }
}

fn token_to_event(token: &SgfToken) -> Option<Event> {
    match token {
        SgfToken::Move {
            color: _,
            action: Action::Move(x, y),
        } => Some(Event::Move((x - 1) as usize, (y - 1) as usize)),

        SgfToken::Add {
            color,
            coordinate: (x, y),
        } => Some(Event::Place(
            color_to_stone(*color),
            (x - 1) as usize,
            (y - 1) as usize,
        )),

        SgfToken::Square { coordinate: (x, y) } => Some(Event::Mark(
            Marker::Square,
            (x - 1) as usize,
            (y - 1) as usize,
        )),

        SgfToken::Triangle { coordinate: (x, y) } => Some(Event::Mark(
            Marker::Triangle,
            (x - 1) as usize,
            (y - 1) as usize,
        )),

        SgfToken::Label {
            label,
            coordinate: (x, y),
        } => Some(Event::Mark(
            Marker::Label(label.chars().next().unwrap()),
            (x - 1) as usize,
            (y - 1) as usize,
        )),

        SgfToken::Comment(s) => Some(Event::Comment(s.clone())),

        _ => None,
    }
}

fn token_to_info(token: &SgfToken, info: &mut GameInfo, size: &mut (usize, usize)) {
    match token {
        SgfToken::Event(s) => info.event = s.to_string(),
        SgfToken::GameName(s) => info.name = s.to_string(),
        SgfToken::PlayerName {
            color: Color::Black,
            name,
        } => info.black_player = name.to_string(),
        SgfToken::PlayerName {
            color: Color::White,
            name,
        } => info.white_player = name.to_string(),
        SgfToken::Size(w, h) => *size = (*w as usize, *h as usize),

        SgfToken::GameComment(s) => info.comment = s.clone(),

        _ => {}
    }
}
