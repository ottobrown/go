use eframe::egui;
use egui::Align;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::game::{NewGameBuilder, GameInfo};
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

    // Editor frame
    egui::Frame::group(&ui.style()).show(ui, |ui| {
        // render player info
        egui::Grid::new("Player info")
            .min_col_width(size.x / 2.0)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(Align::Min), |ui| {
                    ui.label(&game.info.black_player);
                    ui.label(game.info.black_rank.display());
                });

                ui.with_layout(egui::Layout::top_down(Align::Max), |ui| {
                    ui.label(&game.info.white_player);
                    ui.label(game.info.white_rank.display());
                });
            });

    
        ui.label("Select tool:");
        egui::ComboBox::from_id_source("Tool selector")
            .selected_text(format!("{:?}", editor.tool))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut editor.tool, Tool::Move, "Move");
                ui.selectable_value(&mut editor.tool, Tool::Place, "Place");
            });

        ui.horizontal(|ui| {
            if ui.button("Pass").clicked() {
                game.handle_event(&Event::Pass);
            }
            if ui.button("Resign").clicked() {
                game.handle_event(&Event::Resign(game.turn))
            }
            if ui.button("Undo").clicked() {
                game.undo();
            }

            egui::ComboBox::from_id_source("Game info editor")
                .selected_text("Game info")
                .width(size.x / 5.0)
                .show_ui(ui, |ui| {
                    edit_game_info(ui, &mut game.info);
                });
        });

        // Board Frame
        egui::Frame::canvas(&ui.style()).show(ui, |ui| {
            let response = render_board(ui, &game.current_board(), style, size, &mut editor.computed);
            handle_click(ui, editor.tool, &response, &editor.computed, &mut game);
        });
    });

    return game;
}

pub fn build_game(ui: &mut Ui, builder: &mut NewGameBuilder) -> Option<Game> {
    egui::ComboBox::from_label("Board size")
        .selected_text(format!("{}x{}", builder.size.0, builder.size.1))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut builder.size, (19, 19), "19x19");
            ui.selectable_value(&mut builder.size, (13, 13), "13x13");
            ui.selectable_value(&mut builder.size, (9, 9), "9x9");

            ui.label("Custom size:");
            ui.add(egui::Slider::new(&mut builder.size.0, 5..=50));
            ui.add(egui::Slider::new(&mut builder.size.1, 5..=50));
        });

    edit_game_info(ui, &mut builder.info);

    if ui.button("Build").clicked() {
        return Some(builder.build());
    }

    return None;
}

fn edit_game_info(ui: &mut Ui, info: &mut GameInfo) {
    ui.label("Name:");
    ui.text_edit_singleline(&mut info.name);

    ui.label("Event:");
    ui.text_edit_singleline(&mut info.event);

    ui.label("Comment:");
    ui.text_edit_singleline(&mut info.comment);

    ui.horizontal(|ui| {
        ui.label("Black player:");
        ui.text_edit_singleline(&mut info.black_player);
    });

    ui.horizontal(|ui| {
        ui.label("Black rank:");
        ui.label(info.black_rank.display());
    });
    ui.add(egui::Slider::new(&mut info.black_rank.0, -30..=9).show_value(false));

    ui.horizontal(|ui| {
        ui.label("White player:");
        ui.text_edit_singleline(&mut info.white_player);
    });

    ui.horizontal(|ui| {
        ui.label("White rank:");
        ui.label(info.white_rank.display());
    });
    ui.add(egui::Slider::new(&mut info.white_rank.0, -30..=9).show_value(false));
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
                }
            };

            game.handle_event(&play);
        }
    }
}
