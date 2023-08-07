use super::shapes;
use crate::board::Markup;
use crate::sgf::Action;
use crate::Stone;
use crate::UiTool;

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

    // draw stones and markup
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

            match board.get_markup(x, y) {
                Markup::Empty => {}
                Markup::Circle => {
                    shapes::circle(&painter, center, stone_radius);
                }
                Markup::Cross => {
                    shapes::cross(&painter, center, stone_radius);
                }
                Markup::Square => {
                    shapes::square(&painter, center, stone_radius);
                }
                Markup::Triangle => {
                    shapes::triangle(&painter, center, stone_radius);
                }
                Markup::Dim => {
                    shapes::dim(&painter, center, spacing);
                }
                Markup::Arrow(end_x, end_y) => {
                    let end = egui::Pos2 {
                        x: inner_rect.min.x + spacing.x * (end_x as f32),
                        y: inner_rect.min.y + spacing.y * (end_y as f32),
                    };
                    shapes::arrow(&painter, center, end);
                }
                Markup::Line(end_x, end_y) => {
                    let end = egui::Pos2 {
                        x: inner_rect.min.x + spacing.x * (end_x as f32),
                        y: inner_rect.min.y + spacing.y * (end_y as f32),
                    };
                    shapes::line(&painter, center, end);
                }
                Markup::Label(s) => {
                    shapes::label(&painter, &s, center, stone_radius);
                }
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
    tool: &mut crate::UiTool,
    turn: &mut Stone,
) -> Action {
    if !br.response.clicked() {
        return Action::NoOp;
    }

    let (w, h) = board.size();

    if let Some(p) = ui.input(|i| i.pointer.interact_pos()) {
        let (x, y) = (
            (((p.x - br.inner_rect.min.x) / br.spacing.x).round() as usize).min(w - 1),
            (((p.y - br.inner_rect.min.y) / br.spacing.y).round() as usize).min(h - 1),
        );

        match tool {
            UiTool::Play => {
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

            /// TODO: return markup `Action`s from this function
            UiTool::Circle => {
                board.set_markup(x, y, Markup::Circle);
            }
            UiTool::Cross => {
                board.set_markup(x, y, Markup::Cross);
            }
            UiTool::Square => {
                board.set_markup(x, y, Markup::Square);
            }
            UiTool::Triangle => {
                board.set_markup(x, y, Markup::Triangle);
            }
            UiTool::Dim => {
                board.set_markup(x, y, Markup::Dim);
            }
            UiTool::Arrow(o) => {
                if let Some((start_x, start_y)) = o {
                    board.set_markup(*start_x, *start_y, Markup::Arrow(x, y));
                    *tool = UiTool::Arrow(None);
                } else {
                    *tool = UiTool::Arrow(Some((x, y)));
                }
            }
            UiTool::Line(o) => {
                if let Some((start_x, start_y)) = o {
                    board.set_markup(*start_x, *start_y, Markup::Line(x, y));
                    *tool = UiTool::Line(None);
                } else {
                    *tool = UiTool::Line(Some((x, y)));
                }
            }
            UiTool::Label => {
                board.set_markup(x, y, Markup::Label(String::from("A")));
            }
        }
    }

    Action::NoOp
}
