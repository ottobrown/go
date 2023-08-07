#![cfg_attr(not(debug_assertions), allow(unused))]

use std::ops::DerefMut;
use std::sync::Mutex;

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

#[cfg(debug_assertions)]
static DEBUG_LOG: Mutex<String> = Mutex::new(String::new());

#[cfg(debug_assertions)]
fn log(s: impl Into<String>) {
    DEBUG_LOG
        .lock()
        .unwrap()
        .deref_mut()
        .push_str(&format!("{} \n\n", s.into()))
}

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
    debug_window: bool,
    tool: UiTool,
}

impl State {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            game: None,
            builder: GameBuilder::default(),
            style: ui::BoardStyle::default(),
            debug_window: false,
            tool: UiTool::Play,
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
                ui.horizontal(|ui| {
                    ui::render(self, ui, frame.info().window_info.size);
                });
            });
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UiTool {
    /// Place alternating black and white stones
    Play,
    Circle,
    Cross,
    Square,
    Triangle,
    Dim,
    Label,
    /// Contains the base of the line
    Arrow(Option<(usize, usize)>),
    Line(Option<(usize, usize)>),
}
