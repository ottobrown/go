use eframe::egui;
use eframe::Frame;
use egui::Context;
use egui::Style;

use crate::State;
use crate::state::OpenGame;
use crate::Game;

mod board;
mod editor;
mod shapes;

pub use board::BoardStyle;
pub use editor::Editor;

pub fn render(state: &mut State, ctx: &Context, _frame: &Frame) {
    ctx.set_style(default_style());

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                match &mut state.game {
                    OpenGame::Open(g) => {
                        if ui.button("Close game").clicked() {
                            state.game = OpenGame::Closed(Game::builder());
                            return;
                        }

                        // Update game
                        state.game = OpenGame::Open(editor::edit_game(
                            ui,
                            g,
                            &state.style,
                            &mut state.editor,
                        ));
                    }

                    OpenGame::Closed(builder) => {
                        match editor::build_game(ui, builder) {
                            Some(g) => state.game = OpenGame::Open(g),
                            None => {}
                        }
                    },
                }
            })
        })
    });
}

fn default_style() -> egui::Style {
    Style {
        visuals: egui::Visuals {
            dark_mode: true,

            ..Default::default()
        },

        ..Default::default()
    }
}
