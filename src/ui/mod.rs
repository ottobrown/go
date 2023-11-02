use std::ops::DerefMut;

use eframe::egui;
use egui::{vec2, Ui, Vec2};

use crate::sgf::Action;
use crate::{Game, State, Stone};

mod board;
mod sgf;
mod shapes;
mod tool;
mod markup;

pub use board::BoardStyle;
pub use tool::*;

pub struct UiState {
    style: BoardStyle,
    debug_window: bool,
    tool: UiTool,
    /// Index on the current node
    comment: Option<usize>,
}
impl Default for UiState {
    fn default() -> Self {
        Self {
            style: BoardStyle::default(),
            debug_window: false,
            tool: UiTool {
                tool: ToolType::Play,
                base: None,
                letter: 'A',
                number: 1,
            },
            comment: None,
        }
    }
}
impl UiState {
    fn clear_comment(&mut self) {
        self.comment = None;
    }
}

pub fn render(state: &mut State, ui: &mut Ui, size: Vec2) {
    if state.game.is_none() {
        ui.vertical(|ui| {
            if game_creator(&mut state.builder, ui) {
                state.game = Some(state.builder.build());
            }
        });
    } else {
        let game_mut = state.game.as_mut().unwrap();
        let a = render_game(&mut state.ui_state, game_mut, ui, size);

        if a != crate::sgf::Action::NoOp {
            let n = crate::util::new_node(&a);
            if n {
                game_mut.board.clear_markup();
                state.ui_state.tool.clear();
                state.ui_state.clear_comment();
            }
            game_mut.tree.handle_new_action(a, n);
        }

        #[cfg(debug_assertions)]
        debug_window(ui, state);
    }
}

fn render_game(state: &mut UiState, game_mut: &mut Game, ui: &mut Ui, size: Vec2) -> Action {
    let min_size = size.x.min(size.y);
    let size = vec2(min_size, min_size);

    let board_render = board::BoardRenderer::build(ui, &game_mut.board, size, &state.style);
    board_render.render_board(&game_mut.board, &state.style);

    let mut a =
        board_render.handle_click(ui, &mut game_mut.board, &mut state.tool, &mut game_mut.turn);

    // TODO: put these in the center of the screen vertically
    ui.vertical(|ui| {
        sidebar(ui, state, game_mut, &mut a);
        sgf::edit_comment(ui, &mut game_mut.tree.current_node_mut().actions, state);
    });

    a
}

fn sidebar(ui: &mut Ui, state: &mut UiState, game_mut: &mut Game, a: &mut Action) {
    if ui.button("save").clicked() {
        if let Err(e) = game_mut.write_to_file() {
            ui.label("FAILED TO SAVE!!");

            #[cfg(debug_assertions)]
            crate::log(format!("Failed to save with {:?}", e));
        }
    }

    if ui.button("pass").clicked() {
        if game_mut.turn == Stone::Black {
            *a = Action::PassBlack;
        }
        if game_mut.turn == Stone::White {
            *a = Action::PassWhite;
        }

        game_mut.turn = !game_mut.turn;
    }

    egui::ComboBox::from_label("Tool")
        .selected_text(format!("{:?}", state.tool.tool))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut state.tool.tool, ToolType::Play, "Play");
            ui.selectable_value(&mut state.tool.tool, ToolType::Circle, "Circle");
            ui.selectable_value(&mut state.tool.tool, ToolType::Cross, "Cross");
            ui.selectable_value(&mut state.tool.tool, ToolType::Square, "Square");
            ui.selectable_value(&mut state.tool.tool, ToolType::Triangle, "Triangle");
            ui.selectable_value(&mut state.tool.tool, ToolType::Dim, "Dim");
            ui.selectable_value(&mut state.tool.tool, ToolType::Arrow, "Arrow");
            ui.selectable_value(&mut state.tool.tool, ToolType::Line, "Line");
            ui.selectable_value(&mut state.tool.tool, ToolType::Number, "Number");
            ui.selectable_value(&mut state.tool.tool, ToolType::Letter, "Letter");
        });

    if cfg!(debug_assertions) {
        ui.checkbox(&mut state.debug_window, "show debug window");
    }

    if sgf::sgf_arrows(ui, game_mut) {
        state.tool.clear();
        state.clear_comment();
    }
}

/// Edits details of the game such as the baord size, etc.
/// returns true if ready to start playing
fn game_creator(builder: &mut crate::GameBuilder, ui: &mut Ui) -> bool {
    if ui.button("open file:").clicked() {
        builder.path = rfd::FileDialog::new()
            .add_filter("sgf", &["sgf"])
            .pick_file();

        if builder.path.is_some() {
            return true;
        }
    }
    ui.separator();

    ui.label("board width");
    ui.add(egui::Slider::new(&mut builder.size.0, 5..=52));

    ui.label("board height");
    ui.add(egui::Slider::new(&mut builder.size.1, 5..=52));

    if ui.button("finish").clicked() {
        return true;
    }

    false
}

fn debug_window(ui: &mut Ui, state: &State) {
    let game = state.game.as_ref().unwrap();

    if state.ui_state.debug_window {
        egui::Window::new("debug").show(ui.ctx(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                egui::CollapsingHeader::new("Game Tree").show(ui, |ui| {
                    ui.code_editor(&mut format!("{:#?}", game.tree));
                });
                ui.code_editor(&mut game.tree.to_text());
                ui.code_editor(crate::DEBUG_LOG.lock().unwrap().deref_mut())
            });
        });
    }
}
