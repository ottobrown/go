use eframe::egui;
use egui::{vec2, Ui, Vec2};

use crate::State;

mod board;
pub use board::BoardStyle;

pub fn render(state: &mut State, ui: &mut Ui, size: Vec2) {
    let min_size = size.x.min(size.y);
    board::render_board(
        &mut state.game.board,
        ui,
        vec2(min_size, min_size),
        state.style,
        &mut state.game.turn,
    );
}
