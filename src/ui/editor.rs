use eframe::egui;
use egui::Ui;

use super::board::{render_board, BoardStyle, Computed};
use crate::game::Marker;
use crate::game::{GameInfo, NewGameBuilder};
use crate::rules::EndGame;
use crate::rules::Rules;
use crate::{Event, Game, Stone};

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tool {
    Move,
    Place,

    Triangle,
    Circle,
    Square,
    Cross,

    Line,
    Arrow,

    Letter,
    Number,
    CustomLabel,
}

pub struct Editor {
    computed: Computed,
    tool: Tool,
    game_info_open: bool,

    line_starting_point: Option<(usize, usize)>,
    arrow_starting_point: Option<(usize, usize)>,

    last_number: u8,

    /// Index on [LETTERS]
    last_letter_index: usize,
    custom_char: String,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            computed: Computed::blank(),
            tool: Tool::Move,
            game_info_open: false,
            line_starting_point: None,
            arrow_starting_point: None,
            last_number: 0,
            last_letter_index: 0,
            custom_char: String::from('A'),
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
            edit_comment(ui, game.history.get_current_event_mut());

            let board = render_board(ui, &game.current_board(), style, size, &mut editor.computed);

            if !game.ended() {
                if let Some(e) = tool(ui, editor, &board, &game) {
                    game.handle_event(&e);
                }
            } else if game.info.end_game != EndGame::NotOver {
                ui.label(game.info.end_game.display());
            } else {
                ui.label("Game over. Edit the result of the game in `Game info`.");
            }
        });

        ui.vertical(|ui| {
            ui.label(&game.info.white_player);
            ui.label(game.info.white_rank.display());
            ui.label(format!("Captures: {}", game.black_prisoners()));
        });
    });

    ui.label(format!("{:#?}", game.history));

    return game;
}

pub fn edit_comment(ui: &mut Ui, current_event: &mut Event) {
    let mut s = current_event.comment().unwrap_or_default();

    ui.text_edit_multiline(&mut s);

    if !s.is_empty() {
        current_event.add_comment(s.clone());
    }
}

fn editor_buttons(ui: &mut Ui, editor: &mut Editor, game: &mut Game) {
    ui.horizontal(|ui| {
        ui.label("Select tool:");
        egui::ComboBox::from_id_source("Tool selector")
            .selected_text(format!("{:?}", editor.tool))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut editor.tool, Tool::Move, "Move");
                ui.selectable_value(&mut editor.tool, Tool::Place, "Place");
                ui.selectable_value(&mut editor.tool, Tool::Triangle, "Triangle");
                ui.selectable_value(&mut editor.tool, Tool::Circle, "Circle");
                ui.selectable_value(&mut editor.tool, Tool::Square, "Square");
                ui.selectable_value(&mut editor.tool, Tool::Cross, "Cross");
                ui.selectable_value(&mut editor.tool, Tool::Line, "Line");
                ui.selectable_value(&mut editor.tool, Tool::Arrow, "Arrow");
                ui.selectable_value(&mut editor.tool, Tool::Letter, "Letter");
                ui.selectable_value(&mut editor.tool, Tool::Number, "Number");
                ui.selectable_value(&mut editor.tool, Tool::CustomLabel, "Custom label");
            });
    });

    ui.horizontal(|ui| {
        if ui.button("Pass").clicked() && !game.ended() {
            game.handle_event(&Event::Pass);
        }
        if ui.button("Resign").clicked() && !game.ended() {
            game.handle_event(&Event::Resign(game.turn))
        }
        if ui.button("Undo").clicked() {
            game.undo();
        }
        if ui.button("Game info").clicked() {
            editor.game_info_open = true;
        }

        if editor.tool == Tool::CustomLabel {
            ui.text_edit_singleline(&mut editor.custom_char);
            match editor.custom_char.chars().next() {
                Some(c) => editor.custom_char = String::from(c),
                None => {}
            };
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

        match crate::sgf::parse_sgf(p.clone(), builder) {
            Ok(()) => return Some(builder.build()),
            Err(e) => {
                ui.label(format!("{e}"));
            }
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

    ui.label("End Game");
    egui::ComboBox::from_id_source("End game editor")
        .selected_text(info.end_game.display())
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut info.end_game, EndGame::NotOver, "Not over");
            ui.selectable_value(&mut info.end_game, EndGame::Draw, "Draw");
            ui.selectable_value(
                &mut info.end_game,
                EndGame::Resign(Stone::Black),
                "Resignation",
            );
            ui.selectable_value(&mut info.end_game, EndGame::Score(Stone::Black, 0), "Score");
            ui.selectable_value(&mut info.end_game, EndGame::Time(Stone::Black), "Time");
            ui.selectable_value(
                &mut info.end_game,
                EndGame::Forfiet(Stone::Black),
                "Forfiet",
            );
        });

    match &mut info.end_game {
        EndGame::Resign(s) => {
            if let Some(stone) = select_stone(ui) {
                *s = stone;
            }
        }
        EndGame::Time(s) => {
            if let Some(stone) = select_stone(ui) {
                *s = stone;
            }
        }
        EndGame::Forfiet(s) => {
            if let Some(stone) = select_stone(ui) {
                *s = stone;
            }
        }

        EndGame::Score(s, p) => {
            if let Some(stone) = select_stone(ui) {
                *s = stone;
            }

            let before = *p;

            // TODO: replae this with a number input box widget
            ui.style_mut().spacing.slider_width *= 3.0;
            ui.add(
                egui::Slider::new(p, 0..=400)
                    .show_value(false)
                    .integer()
                    .text(format!("{}", 0.5 * (before as f32))),
            );

            ui.horizontal(|ui| {
                if ui.button("-5").clicked() {
                    *p -= 10
                }
                if ui.button("-0.5").clicked() {
                    *p -= 1
                }

                if ui.button("+0.5").clicked() {
                    *p += 1
                }
                if ui.button("+5").clicked() {
                    *p += 10
                }
            });
        }

        _ => {}
    };
}

fn select_stone(ui: &mut Ui) -> Option<Stone> {
    ui.horizontal(|ui| {
        if ui.button("Black").clicked() {
            return Some(Stone::Black);
        }
        if ui.button("White").clicked() {
            return Some(Stone::White);
        }
        return None;
    })
    .inner
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

fn tool(ui: &mut Ui, editor: &mut Editor, board: &egui::Response, game: &Game) -> Option<Event> {
    let c = &editor.computed;

    if board.clicked() || board.secondary_clicked() {
        if let Some(p) = ui.input().pointer.interact_pos() {
            let (x, y) = (
                ((p.x - c.inner_rect.min.x) / c.spacing.x).round() as usize,
                ((p.y - c.inner_rect.min.y) / c.spacing.y).round() as usize,
            );

            let (w, h) = game.size();
            if x * h + y >= w * h {
                return None;
            }

            if editor.tool != Tool::Line {
                editor.line_starting_point = None;
            }

            if board.secondary_clicked() {
                match game.current_board().get_marker(x, y) {
                    None | Some(Marker::Empty) => return Some(Event::Place(Stone::Empty, x, y)),
                    _ => return Some(Event::Mark(Marker::Empty, x, y)),
                }
            }

            return match editor.tool {
                Tool::Move => Some(Event::Move(x, y)),
                Tool::Place => {
                    let mut color = Stone::Black;

                    if ui.input().modifiers.shift {
                        color = Stone::White;
                    }

                    Some(Event::Place(color, x, y))
                }

                Tool::Triangle => Some(Event::Mark(Marker::Triangle, x, y)),
                Tool::Circle => Some(Event::Mark(Marker::Circle, x, y)),
                Tool::Square => Some(Event::Mark(Marker::Square, x, y)),
                Tool::Cross => Some(Event::Mark(Marker::Cross, x, y)),

                Tool::Line => match editor.line_starting_point {
                    Some(p) => {
                        editor.line_starting_point = None;

                        Some(Event::Mark(Marker::Line(x, y), p.0, p.1))
                    }
                    None => {
                        editor.line_starting_point = Some((x, y));

                        None
                    }
                },

                Tool::Arrow => match editor.arrow_starting_point {
                    Some(p) => {
                        editor.arrow_starting_point = None;

                        Some(Event::Mark(Marker::Arrow(x, y), p.0, p.1))
                    }
                    None => {
                        editor.arrow_starting_point = Some((x, y));

                        None
                    }
                },

                Tool::Letter => {
                    if editor.last_letter_index > 25 {
                        editor.last_letter_index = 0;
                    }

                    let c = LETTERS.chars().nth(editor.last_letter_index).unwrap();

                    editor.last_letter_index += 1;

                    Some(Event::Mark(Marker::Label(c), x, y))
                }

                Tool::Number => {
                    editor.last_number += 1;
                    if editor.last_number > 9 {
                        editor.last_number = 1;
                    }

                    let s = format!("{}", editor.last_number);
                    let c = s.chars().next().unwrap();

                    Some(Event::Mark(Marker::Label(c), x, y))
                }

                Tool::CustomLabel => {
                    let c = editor.custom_char.chars().next().unwrap();
                    Some(Event::Mark(Marker::Label(c), x, y))
                }
            };
        }
    }

    return None;
}
