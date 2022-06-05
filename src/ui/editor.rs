use eframe::egui;
use egui::Align;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::game::{NewGameBuilder, GameInfo};
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

pub fn edit_game(ui: &mut Ui, g: &Game, style: &BoardStyle, editor: &mut Editor) -> Game {
    let mut game: Game = g.clone();

    let size = egui::vec2(800.0, 800.0);

    // Editor frame
    egui::Frame::group(&ui.style()).show(ui, |ui| {
        // render player info
        egui::Grid::new("player info")
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

        egui::ComboBox::from_label("")
            .selected_text("Game Info")
            .show_ui(ui, |ui| {
                edit_game_info(ui, &mut game.info);
            });

        // Board Frame
        egui::Frame::canvas(&ui.style()).show(ui, |ui| {
            render_board(ui, &game.current_board(), style, size, &mut editor.computed);
        });
    });

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
            ui.add(egui::Slider::new(&mut builder.size.0, 5..=50));
            ui.add(egui::Slider::new(&mut builder.size.1, 5..=50));
        });

    edit_game_info(ui, &mut builder.info);

    if ui.button("build").clicked() {
        return Some(builder.build());
    }

    return None;
}

fn edit_game_info(ui: &mut Ui, info: &mut GameInfo) {
    ui.label("name:");
    ui.text_edit_singleline(&mut info.name);

    ui.label("event");
    ui.text_edit_singleline(&mut info.event);

    ui.label("comment");
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
