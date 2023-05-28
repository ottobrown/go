use eframe::egui;
use egui::{vec2, Ui, Vec2};

use crate::State;

mod board;
mod sgf;
pub use board::BoardStyle;

pub fn render(state: &mut State, ui: &mut Ui, size: Vec2) {
    let min_size = size.x.min(size.y);

    ui.horizontal(|ui| {
        let br = board::render_board(
            &mut state.game.board,
            ui,
            vec2(min_size, min_size),
            state.style,
        );
        let a = board::handle_click(ui, &br, &mut state.game.board, &mut state.game.turn);

        if a != crate::sgf::Action::NoOp {
            state.game.tree.handle_new_text(a.to_sgf_text().unwrap());
        }

        ui.code_editor(&mut format!("{:#?}", state.game.tree));

        sgf::sgf_arrows(ui, &mut state.game);
    });
}
