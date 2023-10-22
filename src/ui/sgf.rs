use super::UiState;
use crate::sgf::Action;
use crate::Game;
use eframe::egui;
use egui::Ui;

/// returns if anty button was pressed
pub fn sgf_arrows(ui: &mut Ui, game: &mut Game) -> bool {
    let mut pressed = false;
    ui.horizontal(|ui| {
        // left arrow
        if ui.button("\u{23F4}").clicked() {
            let _ = game.tree.select_parent();
            game.do_to_now();
        }

        // right arrows
        ui.vertical(|ui| {
            for i in 0..game.tree.num_children() {
                if ui.button("\u{23F5}").clicked() {
                    pressed = true;

                    let _ = game.tree.select_child(i);
                    game.do_to_now();
                }
            }
        });
    });

    pressed
}

pub fn edit_comment(ui: &mut Ui, actions: &mut Vec<Action>, state: &mut UiState) {
    if let Some(i) = state.comment {
        if let Action::Comment(ref mut s) = &mut actions[i] {
            ui.text_edit_multiline(s);
        } else {
            #[cfg(debug_assertions)]
            crate::log(format!("UiState::comment is not valid!"));

            state.comment = None;
        }
    } else {
        for (i, a) in actions.iter().enumerate() {
            if let Action::Comment(_) = a {
                state.comment = Some(i);
                return;
            }
        }

        let mut s = String::new();
        ui.text_edit_multiline(&mut s);
        if !s.is_empty() {
            state.comment = Some(actions.len());
            actions.push(Action::Comment(s));
        }
    }
}
