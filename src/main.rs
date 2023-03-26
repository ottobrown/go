use eframe::egui;

mod board;
mod ui;

pub use board::Board;
pub use board::Stone;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Go",
        native_options,
        Box::new(|cc| Box::new(State::new(cc))),
    )
}

pub struct State {
    board: Board,
    style: ui::BoardStyle,
}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            board: Board::new(19, 19),
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
