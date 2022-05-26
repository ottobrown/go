use eframe::egui;
use eframe::{CreationContext, Frame};
use egui::Context;

use crate::ui;
use crate::Game;

pub struct State {
    pub game: Game,
    pub editor: ui::Editor,
    pub style: ui::BoardStyle,
}
impl State {
    pub fn new(_cc: &CreationContext) -> Self {
        State {
            game: Game::builder().build(),
            editor: ui::Editor::default(),
            style: ui::BoardStyle::default(),
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ui::render(self, ctx, frame);
    }
}
