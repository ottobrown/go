use std::ops::DerefMut;

use eframe::egui;
use egui::{vec2, Ui, Vec2};

use crate::sgf::Action;
use crate::UiTool;
use crate::{State, Stone};

mod board;
mod sgf;
mod shapes;
pub use board::BoardStyle;

pub fn render(state: &mut State, ui: &mut Ui, size: Vec2) {
    if state.game.is_none() {
        ui.vertical(|ui| {
            if game_creator(&mut state.builder, ui) {
                state.game = Some(state.builder.build());
            }
        });
    } else {
        let min_size = size.x.min(size.y);

        let game_mut = state.game.as_mut().unwrap();

        let br = board::render_board(
            &mut game_mut.board,
            ui,
            vec2(min_size, min_size),
            state.style,
        );
        let mut a = board::handle_click(
            ui,
            &br,
            &mut game_mut.board,
            &mut state.tool,
            &mut game_mut.turn,
        );

        // TODO: put these in the center of the screen vertically
        ui.vertical(|ui| {
            if ui.button("save").clicked() {
                if let Err(e) = game_mut.write_to_file() {
                    ui.label("FAILED TO SAVE!!");

                    #[cfg(debug_assertions)]
                    crate::log(format!("Failed to save with {:?}", e));
                }
            }

            if ui.button("pass").clicked() {
                if game_mut.turn == Stone::Black {
                    a = Action::PassBlack;
                }
                if game_mut.turn == Stone::White {
                    a = Action::PassWhite;
                }

                game_mut.turn = !game_mut.turn;
            }

            egui::ComboBox::from_label("Tool")
                .selected_text(format!("{:?}", state.tool))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.tool, UiTool::Play, "Play");
                    ui.selectable_value(&mut state.tool, UiTool::Circle, "Circle");
                    ui.selectable_value(&mut state.tool, UiTool::Cross, "Cross");
                    ui.selectable_value(&mut state.tool, UiTool::Square, "Square");
                    ui.selectable_value(&mut state.tool, UiTool::Triangle, "Triangle");
                    ui.selectable_value(&mut state.tool, UiTool::Dim, "Dim");
                    ui.selectable_value(&mut state.tool, UiTool::Label, "Label");
                    ui.selectable_value(&mut state.tool, UiTool::Arrow(None), "Arrow");
                    ui.selectable_value(&mut state.tool, UiTool::Line(None), "Line");
                });

            if cfg!(debug_assertions) {
                ui.checkbox(&mut state.debug_window, "show debug window");
            }

            sgf::sgf_arrows(ui, game_mut);
        });

        #[cfg(debug_assertions)]
        if state.debug_window {
            egui::Window::new("debug").show(ui.ctx(), |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::CollapsingHeader::new("Game Tree").show(ui, |ui| {
                        ui.code_editor(&mut format!("{:#?}", game_mut.tree));
                    });
                    ui.code_editor(&mut game_mut.tree.to_text());
                    ui.code_editor(crate::DEBUG_LOG.lock().unwrap().deref_mut())
                });
            });
        }

        if a != crate::sgf::Action::NoOp {
            let n = crate::util::new_node(&a);
            game_mut.tree.handle_new_action(a, n);
        }
    }
}

/// Edits details of the game such as the baord size, etc.
/// returns true if ready to start playing
fn game_creator(builder: &mut crate::GameBuilder, ui: &mut Ui) -> bool {
    if ui.button("open file:").clicked() {
        builder.path = rfd::FileDialog::new()
            .add_filter("sgf", &["sgf"])
            .pick_file();

        if builder.path.is_some() {
            return true;
        }
    }
    ui.separator();

    ui.label("board width");
    ui.add(egui::Slider::new(&mut builder.size.0, 5..=52));

    ui.label("board height");
    ui.add(egui::Slider::new(&mut builder.size.1, 5..=52));

    if ui.button("finish").clicked() {
        return true;
    }

    false
}
