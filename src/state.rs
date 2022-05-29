use eframe::egui;
use eframe::{CreationContext, Frame};
use egui::Context;

use crate::game::NewGameBuilder;
use crate::ui;
use crate::Game;

pub struct State {
    pub game: Option<Game>,
    pub builder: NewGameBuilder,

    pub editor: ui::Editor,
    pub style: ui::BoardStyle,
}
impl State {
    pub fn new(_cc: &CreationContext) -> Self {
        State {
            game: None,
            builder: Game::builder(),
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
