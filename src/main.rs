// Hide terminal on windows
#![windows_subsystem = "windows"]

#![allow(clippy::needless_return)]
#![allow(clippy::redundant_field_names)]

use eframe::{run_native, NativeOptions};

mod board;
mod game;
mod rules;
mod state;
mod tree;
mod ui;
mod sgf;

use board::Board;
use board::Stone;
use game::Event;
use game::Game;
use rules::Rules;
use state::State;

fn main() {
    let ops = NativeOptions::default();

    run_native("Go", ops, Box::new(|cc| Box::new(state::State::new(cc))))
}
