use eframe::egui;

mod board;
mod flood_fill;
mod game;
mod sgf;
mod ui;
mod util;

pub use board::{Board, Stone};
pub use game::{Game, GameBuilder};
pub use sgf::{SgfNode, SgfTree};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Go",
        native_options,
        Box::new(|cc| Box::new(State::new(cc))),
    )
}

pub struct State {
    /// Some => show board editor
    /// None => show game builder ui
    game: Option<Game>,
    builder: GameBuilder,
    style: ui::BoardStyle,
}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            game: None,
            builder: GameBuilder::default(),
            style: ui::BoardStyle::default(),
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                // remove the margins so the `ui::render` function has access to the full window
                egui::Frame::default().inner_margin(0.0).outer_margin(0.0),
            )
            .show(ctx, |ui| {
                ui::render(self, ui, frame.info().window_info.size);
            });
    }
}
