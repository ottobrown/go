use eframe::egui;
use egui::{vec2, Ui};

use crate::State;

mod board;
pub use board::BoardStyle;

pub fn render(state: &mut State, ui: &mut Ui) {
    board::render_board(
        &mut state.game.board,
        ui,
        vec2(800.0, 800.0),
        state.style,
        &mut state.game.turn,
    );
}
