use std::path::PathBuf;

use crate::game::Event;
use crate::tree::EventTree;
use crate::tree::EventNode;

use sgf_parser::{
    GameTree,
    SgfToken,
};

pub fn open_sgf() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("SGF", &["sgf"])
        .pick_file()
}

pub fn parse_sgf(sgf_data: &str) -> Result<EventTree, String> {
    let game_tree = match sgf_parser::parse(sgf_data) {
        Ok(t) => t,
        Err(_) => { return Err(String::from("Failed to parse sgf")) },
    };

    let mut root = EventNode::new(Event::Start);

    fn build_node(game_tree: GameTree, node: &mut EventNode) {
    
    }

    build_node(game_tree, &mut root);
    
    return Ok(EventTree::from_root(root));
}

fn token_to_event(t: &SgfToken) -> Option<Event> {
    match t {

        _ => None,
    }
}
