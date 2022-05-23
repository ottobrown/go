use eframe::egui;
use eframe::Frame;
use egui::Context;
use egui::Style;

use crate::State;

mod board;
mod editor;

pub use board::BoardStyle;
pub use editor::Editor;

pub fn render(state: &mut State, ctx: &Context, _frame: &Frame) {
    ctx.set_style(default_style());

    egui::CentralPanel::default().show(ctx, |ui| {
        editor::edit_game(ui, &mut state.game, &state.style, &mut state.editor);
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
