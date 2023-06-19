use eframe::egui;
use egui::{vec2, Ui, Vec2};

use crate::State;

mod board;
mod sgf;
pub use board::BoardStyle;

pub fn render(state: &mut State, ui: &mut Ui, size: Vec2) {
    if state.game.is_none() {
        if game_creator(&mut state.builder, ui) {
            state.game = Some(state.builder.build());
        }
    } else {
        let min_size = size.x.min(size.y);

        let game_mut = state.game.as_mut().unwrap();

        ui.horizontal(|ui| {
            let br = board::render_board(
                &mut game_mut.board,
                ui,
                vec2(min_size, min_size),
                state.style,
            );
            let a = board::handle_click(ui, &br, &mut game_mut.board, &mut game_mut.turn);

            if a != crate::sgf::Action::NoOp {
                game_mut.tree.handle_new_text(a.to_sgf_text().unwrap());
            }
            sgf::sgf_arrows(ui, game_mut);
            ui.checkbox(&mut state.debug_window, "show debug window");
        });

        if state.debug_window {
            egui::Window::new("debug").show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.code_editor(&mut format!("{:#?}", game_mut.tree));
                });
            });
        }
    }
}

/// Edits details of the game such as the baord size, etc.
/// returns true if ready to start playing
fn game_creator(builder: &mut crate::GameBuilder, ui: &mut Ui) -> bool {
    ui.label("board width");
    ui.add(egui::Slider::new(&mut builder.size.0, 5..=50));

    ui.label("board height");
    ui.add(egui::Slider::new(&mut builder.size.1, 5..=50));

    if ui.button("finish").clicked() {
        return true;
    }

    false
}
