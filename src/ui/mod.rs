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
    egui::Frame::group(ui.style()).show(ui, |ui| {
    egui::ScrollArea::both().show(ui, |ui| {
        if state.game.is_some() {
            state.game = Some(editor::edit_game(
                ui,
                state.game.as_ref().unwrap(),
                &state.style,
                &mut state.editor,
            ));
        } else {
            state.game = editor::build_game(ui, &mut state.builder);
        }
    })})});
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
