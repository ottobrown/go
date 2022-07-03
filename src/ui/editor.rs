use eframe::egui;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::game::{GameInfo, NewGameBuilder};
use crate::rules::Rules;
use crate::{Event, Game, Stone};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tool {
    Move,
    Place,
}

pub struct Editor {
    computed: Computed,
    tool: Tool,
    game_info_open: bool,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            computed: Computed::blank(),
            tool: Tool::Move,
            game_info_open: false,
        }
    }
}

pub fn edit_game(ui: &mut Ui, g: &Game, style: &BoardStyle, editor: &mut Editor) -> Game {
    let mut game: Game = g.clone();

    let size = egui::Vec2::splat(ui.style().spacing.item_spacing.x * 100.0);

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(&game.info.black_player);
            ui.label(game.info.black_rank.display());
            ui.label(format!("Captures: {}", game.white_prisoners()));
        });

        ui.vertical(|ui| {
            editor_buttons(ui, editor, &mut game);
            let r = render_board(ui, &game.current_board(), style, size, &mut editor.computed);
            match &game.end_game {
                Some(e) => {
                    ui.label(e.display());
                }
                None => {
                    handle_click(ui, editor.tool, &r, &editor.computed, &mut game);
                }
            };
        });

        ui.vertical(|ui| {
            ui.label(&game.info.white_player);
            ui.label(game.info.white_rank.display());
            ui.label(format!("Captures: {}", game.black_prisoners()));
        });
    });

    return game;
}

fn editor_buttons(ui: &mut Ui, editor: &mut Editor, game: &mut Game) {
    ui.horizontal(|ui| {
        ui.label("Select tool:");
        egui::ComboBox::from_id_source("Tool selector")
            .selected_text(format!("{:?}", editor.tool))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut editor.tool, Tool::Move, "Move");
                ui.selectable_value(&mut editor.tool, Tool::Place, "Place");
            });
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
        if ui.button("Game info").clicked() {
            editor.game_info_open = true;
        }
    });

    ui.horizontal(|ui| {
        // left arrow
        if ui.button("\u{2B05}").clicked() {
            game.move_back();
        }

        // right arrow
        if ui.button("\u{27A1}").clicked() {
            game.move_forward();
        }

        // up arrow
        if ui.button("\u{2B06}").clicked() {
            game.move_up();
        }

        // down arrow
        if ui.button("\u{2B07}").clicked() {
            game.move_down();
        }
    });

    if editor.game_info_open {
        egui::Window::new("Game info").show(ui.ctx(), |ui| {
            edit_game_info(ui, &mut game.info);

            if ui.button("Close").clicked() {
                editor.game_info_open = false;
            }
        });
    }
}

pub fn build_game(ui: &mut Ui, builder: &mut NewGameBuilder) -> Option<Game> {
    if ui.button("Open sgf").clicked() {
        builder.info.sgf_path = crate::sgf::open_sgf();
    }

    if let Some(p) = &builder.info.sgf_path {
        ui.label(format!("sgf file: {}", p.display()));
        match crate::sgf::parse_tree(p.clone()) {
            Ok(event_tree) => builder.tree = Some(event_tree),
            Err(e) => {
                ui.label(format!("Failed to parse sgf data {}", e));
            },
        };
    }

    egui::Grid::new("Builder grid layout")
        .spacing(ui.style().spacing.item_spacing * 2.0)
        .num_columns(2)
        .show(ui, |ui| {
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

            ui.end_row();

            ui.vertical(|ui| {
                edit_rules(ui, &mut builder.rules);
            });

            ui.vertical(|ui| {
                edit_game_info(ui, &mut builder.info);
            });
        });

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

fn edit_rules(ui: &mut Ui, rules: &mut Rules) {
    ui.heading("Game Rules");

    ui.selectable_value(rules, Rules::CHINESE, "Chinese Rules");
    ui.selectable_value(rules, Rules::JAPANESE, "Japanese Rules");
    ui.selectable_value(rules, Rules::NEW_ZEALAND, "New Zealand Rules");

    ui.checkbox(&mut rules.suicide_allowed, "Suicidal moves allowed");
    ui.checkbox(&mut rules.superko, "Superko");
    ui.label(format!("Komi: {}", rules.komi as f32 * 0.5));
    ui.add(egui::Slider::new(&mut rules.komi, 0..=50).show_value(false));
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
