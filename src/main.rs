use eframe::egui;

mod board;
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
                board: Board::new(19, 19),
                turn: Stone::Black,
            },
            style: ui::BoardStyle::default(),
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::render(self, ui);
        });
    }
}
