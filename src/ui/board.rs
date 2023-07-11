use crate::sgf::Action;
use crate::Stone;

use eframe::egui;
use egui::{pos2, vec2, Color32, Ui};

#[derive(Clone, Copy)]
pub struct BoardStyle {
    background_color: Color32,
    /// as a proportion of the board width/height
    padding: f32,
    line_thickness: f32,
    star_point_radius: f32,
    /// as a proportion of the min of the spacing between
    /// horizontal lines and the spacing between vertical lines
    stone_radius: f32,
}

impl Default for BoardStyle {
    fn default() -> Self {
        Self {
            background_color: Color32::from_rgb(0xDE, 0xB8, 0x87),
            padding: 0.05,
            line_thickness: 2.0,
            star_point_radius: 5.0,
            stone_radius: 0.4,
        }
    }
}

pub(super) fn render_board(
    board: &mut crate::Board,
    ui: &mut Ui,
    size: egui::Vec2,
    style: BoardStyle,
) -> BoardResponse {
    let (response, painter) = ui.allocate_painter(size, egui::Sense::drag());
    let (w, h) = board.size();

    // Draw background color
    painter.rect_filled(
        egui::Rect::EVERYTHING,
        egui::Rounding::none(),
        style.background_color,
    );

    let pos = response.rect.min;

    let padding = egui::Vec2 {
        x: response.rect.width() * style.padding,
        y: response.rect.height() * style.padding,
    };

    let inner_rect = egui::Rect::from_min_size(
        pos2(pos.x + padding.x, pos.y + padding.y),
        vec2(
            response.rect.width() - 2.0 * padding.x,
            response.rect.height() - 2.0 * padding.y,
        ),
    );

    let spacing = vec2(
        inner_rect.width() / (w - 1) as f32,
        inner_rect.height() / (h - 1) as f32,
    );

    // draw outermost lines
    painter.rect_stroke(
        inner_rect,
        egui::Rounding::none(),
        egui::Stroke::new(style.line_thickness, Color32::BLACK),
    );

    // draw vertical lines
    for x in 0..w {
        let x_pos = inner_rect.min.x + spacing.x * (x as f32);
        let start_y = inner_rect.min.y;
        let end_y = inner_rect.max.y;

        painter.line_segment(
            [pos2(x_pos, start_y), pos2(x_pos, end_y)],
            egui::Stroke::from((style.line_thickness, Color32::BLACK)),
        );
    }

    // draw horizontal lines
    for y in 0..h {
        let y_pos = inner_rect.min.y + spacing.y * (y as f32);
        let start_x = inner_rect.min.x;
        let end_x = inner_rect.max.x;

        painter.line_segment(
            [pos2(start_x, y_pos), pos2(end_x, y_pos)],
            egui::Stroke::from((style.line_thickness, Color32::BLACK)),
        );
    }

    for p in crate::util::star_points(w, h) {
        let pos = egui::Pos2 {
            x: inner_rect.min.x + spacing.x * (p.0 as f32),
            y: inner_rect.min.y + spacing.y * (p.1 as f32),
        };
        painter.circle_filled(pos, style.star_point_radius, Color32::BLACK);
    }

    let stone_radius = f32::min(spacing.x, spacing.y) * style.stone_radius;

    // draw stones
    for x in 0..w {
        for y in 0..h {
            let center = egui::Pos2 {
                x: inner_rect.min.x + spacing.x * (x as f32),
                y: inner_rect.min.y + spacing.y * (y as f32),
            };

            if board.get(x, y) == crate::Stone::Black {
                painter.circle_filled(center, stone_radius, Color32::BLACK);
            }

            if board.get(x, y) == crate::Stone::White {
                painter.circle_filled(center, stone_radius, Color32::WHITE);
            }
        }
    }

    BoardResponse {
        response,
        inner_rect,
        spacing,
    }
}

pub(super) struct BoardResponse {
    response: egui::Response,
    inner_rect: egui::Rect,
    spacing: egui::Vec2,
}

pub(super) fn handle_click(
    ui: &mut Ui,
    br: &BoardResponse,
    board: &mut crate::Board,
    turn: &mut Stone,
) -> Action {
    let (w, h) = board.size();
    if br.response.clicked() {
        if let Some(p) = ui.input(|i| i.pointer.interact_pos()) {
            let (x, y) = (
                (((p.x - br.inner_rect.min.x) / br.spacing.x).round() as usize).min(w - 1),
                (((p.y - br.inner_rect.min.y) / br.spacing.y).round() as usize).min(h - 1),
            );

            if board.attempt_set(x, y, *turn) {
                *turn = !*turn;

                if !*turn == Stone::Black {
                    return Action::PlayBlack(x, y);
                }
                if !*turn == Stone::White {
                    return Action::PlayWhite(x, y);
                }
            }
        }
    }

    if ui.button("pass").clicked() {
        *turn = !*turn;
        if !*turn == Stone::Black {
            return Action::PassBlack;
        }
        if !*turn == Stone::White {
            return Action::PassWhite;
        }
    }

    Action::NoOp
}
