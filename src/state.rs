use eframe::egui;
use eframe::{CreationContext, Frame};
use egui::Context;

use crate::ui;

pub struct State {}
impl State {
    pub fn new(_cc: &CreationContext) -> Self {
        State {}
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ui::render(self, ctx, frame);
    }
}
