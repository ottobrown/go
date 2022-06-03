use eframe::egui;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::game::NewGameBuilder;
use crate::{Event, Game, Stone};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tool {
    Move,
    Place,
}

pub struct Editor {
    pub computed: Computed,
    tool: Tool,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            computed: Computed::blank(),
            tool: Tool::Move,
        }
    }
}

pub fn edit_game(ui: &mut Ui, g: &Game, style: &BoardStyle, editor: &mut Editor) -> Game {
    let mut game: Game = g.clone();

    let size = egui::vec2(800.0, 800.0);

    if ui.button("Pass").clicked() {
        game.handle_event(&Event::Pass);
    }
    if ui.button("Resign").clicked() {
        game.handle_event(&Event::Resign(game.turn))
    }
    // TODO: add undo button

    egui::ComboBox::from_label("Select tool")
        .selected_text(format!("{:?}", editor.tool))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut editor.tool, Tool::Move, "Move");
            ui.selectable_value(&mut editor.tool, Tool::Place, "Place");
        });

    let response = render_board(ui, &game.current_board(), style, size, &mut editor.computed);
    handle_click(ui, editor.tool, &response, &editor.computed, &mut game);

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

fn handle_click(ui: &mut Ui, tool: Tool, response: &egui::Response, c: &Computed, game: &mut Game) {
    if response.clicked() || response.secondary_clicked() {
        if let Some(p) = ui.input().pointer.interact_pos() {
            let (x, y) = (
                ((p.x - c.inner_rect.min.x) / c.spacing.x).round() as usize,
                ((p.y - c.inner_rect.min.y) / c.spacing.y).round() as usize,
            );

            let (w, h) = game.size();
            if (x as usize) * h + (y as usize) >= w * h {
                return;
            }

            let play = match tool {
                Tool::Move => Event::Move(x as usize, y as usize),
                Tool::Place => {
                    let mut color = Stone::Black;

                    if ui.input().modifiers.shift {
                        color = Stone::White;
                    }
                    if response.secondary_clicked() {
                        color = Stone::Empty;
                    }

                    Event::Place(color, x as usize, y as usize)
                },
            };

            game.handle_event(&play);
        }
    }
}
