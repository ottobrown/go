use eframe::egui;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::{Event, Game};

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

    handle_click(ui, &editor.computed, game);
}

pub fn handle_click(ui: &mut Ui, c: &Computed, game: &mut Game) {
    if ui.input().pointer.primary_down() {
        if let Some(p) = ui.input().pointer.interact_pos() {
            let (x, y) = (
                ((p.x - c.inner_rect.min.x) / c.spacing.x).round() as usize,
                ((p.y - c.inner_rect.min.y) / c.spacing.y).round() as usize,
            );

            let s = game.size();
            if (x as usize) * s.0 + (y as usize) >= s.0 * s.1 {
                return;
            }

            let play = Event::Move(x, y);
            game.handle_event(&play);
        }
    }
}
