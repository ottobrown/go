use crate::Game;
use eframe::egui;
use egui::Ui;

pub fn sgf_arrows(ui: &mut Ui, game: &mut Game) {
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
                    let _ = game.tree.select_child(i);
                    game.do_to_now();
                }
            }
        });
    });
}
