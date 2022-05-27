use eframe::egui;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::{Event, Game};
use crate::game::NewGameBuilder;

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

pub fn edit_game(ui: &mut Ui, g: &Game, style: &BoardStyle, editor: &mut Editor) -> Game {
    let mut game: Game = g.clone();

    let size = egui::vec2(800.0, 800.0);

    render_board(ui, &game.current_board(), style, size, &mut editor.computed);
    handle_click(ui, &editor.computed, &mut game);

    return game;
}

pub fn build_game(ui: &mut Ui, builder: &mut NewGameBuilder) -> Option<Game> {
    egui::ComboBox::from_label("Board Size")
        .selected_text(format!("{}x{}", builder.size.0, builder.size.1))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut builder.size, (19, 19), "19x19");
            ui.selectable_value(&mut builder.size, (13, 13), "13x13");
            ui.selectable_value(&mut builder.size, (9, 9), "9x9");

            ui.label("custom size:");
            ui.add(egui::Slider::new(&mut builder.size.0, 5..=50).text("Board Width"));
            ui.add(egui::Slider::new(&mut builder.size.1, 5..=50).text("Board Height"));
        });
    
    if ui.button("build").clicked() {
        return Some(builder.build());
    }

    None
}

fn handle_click(ui: &mut Ui, c: &Computed, game: &mut Game) {
    if ui.input().pointer.primary_down() {
        if let Some(p) = ui.input().pointer.interact_pos() {
            let (x, y) = (
                ((p.x - c.inner_rect.min.x) / c.spacing.x).round() as usize,
                ((p.y - c.inner_rect.min.y) / c.spacing.y).round() as usize,
            );

            let (w, h) = game.size();
            if (x as usize) * h + (y as usize) >= w * h {
                return;
            }

            let play = Event::Move(x, y);
            game.handle_event(&play);
        }
    }
}
