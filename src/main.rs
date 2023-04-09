use eframe::egui;

mod board;
mod flood_fill;
mod game;
mod ui;

pub use board::Board;
pub use board::Stone;
pub use game::Game;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Go",
        native_options,
        Box::new(|cc| Box::new(State::new(cc))),
    )
}

pub struct State {
    game: Game,
    style: ui::BoardStyle,
}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            game: Game {
                // TODO: non-square boards don't place stones correctly
                board: Board::new(19, 19),
                turn: Stone::Black,
            },
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
