use eframe::egui;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::Game;

pub struct Editor {
    pub computed: Computed,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            computed: Computed::blank(),
        }
    }
}

pub fn edit_game(ui: &mut Ui, game: &mut Game, style: &BoardStyle, editor: &mut Editor) {
    let size = egui::vec2(800.0, 800.0);

    render_board(ui, &game.current_board(), style, size, &mut editor.computed);
}
