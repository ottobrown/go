use eframe::{run_native, NativeOptions};

mod board;
mod game;
mod state;
mod ui;

pub use board::Board;
pub use state::State;

fn main() {
    let ops = NativeOptions::default();

    run_native("Go", ops, Box::new(|cc| Box::new(state::State::new(cc))))
}
