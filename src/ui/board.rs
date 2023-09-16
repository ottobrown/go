use super::shapes;
use super::ToolType;
use super::UiTool;
use crate::board::Markup;
use crate::sgf::Action;
use crate::Board;
use crate::Stone;

use eframe::egui;
use egui::{pos2, vec2, Color32, Pos2, Ui};

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

pub(super) struct BoardRenderer {
    response: egui::Response,
    painter: egui::Painter,
    inner_rect: egui::Rect,
    spacing: egui::Vec2,
    stone_radius: f32,
}
impl BoardRenderer {
    pub fn build(ui: &mut Ui, board: &Board, size: egui::Vec2, style: &BoardStyle) -> Self {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::drag());

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

        let (w, h) = board.size();

        let spacing = vec2(
            inner_rect.width() / (w - 1) as f32,
            inner_rect.height() / (h - 1) as f32,
        );

        let stone_radius = f32::min(spacing.x, spacing.y) * style.stone_radius;

        Self {
            response,
            painter,
            inner_rect,
            spacing,
            stone_radius,
        }
    }

    pub fn render_board(&self, board: &Board, style: &BoardStyle) {
        let (w, h) = board.size();

        // Draw background color
        self.painter.rect_filled(
            egui::Rect::EVERYTHING,
            egui::Rounding::none(),
            style.background_color,
        );

        // draw outermost lines
        self.painter.rect_stroke(
            self.inner_rect,
            egui::Rounding::none(),
            egui::Stroke::new(style.line_thickness, Color32::BLACK),
        );

        // draw vertical lines
        for x in 0..w {
            let x_pos = self.inner_rect.min.x + self.spacing.x * (x as f32);
            let start_y = self.inner_rect.min.y;
            let end_y = self.inner_rect.max.y;

            self.painter.line_segment(
                [pos2(x_pos, start_y), pos2(x_pos, end_y)],
                egui::Stroke::from((style.line_thickness, Color32::BLACK)),
            );
        }

        // draw horizontal lines
        for y in 0..h {
            let y_pos = self.inner_rect.min.y + self.spacing.y * (y as f32);
            let start_x = self.inner_rect.min.x;
            let end_x = self.inner_rect.max.x;

            self.painter.line_segment(
                [pos2(start_x, y_pos), pos2(end_x, y_pos)],
                egui::Stroke::from((style.line_thickness, Color32::BLACK)),
            );
        }

        for p in crate::util::star_points(w, h) {
            let pos = egui::Pos2 {
                x: self.inner_rect.min.x + self.spacing.x * (p.0 as f32),
                y: self.inner_rect.min.y + self.spacing.y * (p.1 as f32),
            };
            self.painter
                .circle_filled(pos, style.star_point_radius, Color32::BLACK);
        }

        // draw stones
        for x in 0..w {
            for y in 0..h {
                let center = egui::Pos2 {
                    x: self.inner_rect.min.x + self.spacing.x * (x as f32),
                    y: self.inner_rect.min.y + self.spacing.y * (y as f32),
                };

                if board.get(x, y) == crate::Stone::Black {
                    self.painter
                        .circle_filled(center, self.stone_radius, Color32::BLACK);
                }

                if board.get(x, y) == crate::Stone::White {
                    self.painter
                        .circle_filled(center, self.stone_radius, Color32::WHITE);
                }
            }
        }

        // draw markup above stones
        for x in 0..w {
            for y in 0..h {
                let center = egui::Pos2 {
                    x: self.inner_rect.min.x + self.spacing.x * (x as f32),
                    y: self.inner_rect.min.y + self.spacing.y * (y as f32),
                };

                self.draw_markup(board.get_markup(x, y), center);
            }
        }
    }

    fn draw_markup(&self, markup: Markup, center: Pos2) {
        match markup {
            Markup::Empty => {}
            Markup::Circle => {
                shapes::circle(&self.painter, center, self.stone_radius);
            }
            Markup::Cross => {
                shapes::cross(&self.painter, center, self.stone_radius);
            }
            Markup::Square => {
                shapes::square(&self.painter, center, self.stone_radius);
            }
            Markup::Triangle => {
                shapes::triangle(&self.painter, center, self.stone_radius);
            }
            Markup::Dim => {
                shapes::dim(&self.painter, center, self.spacing);
            }
            Markup::Arrow(end_x, end_y) => {
                let end = egui::Pos2 {
                    x: self.inner_rect.min.x + self.spacing.x * (end_x as f32),
                    y: self.inner_rect.min.y + self.spacing.y * (end_y as f32),
                };
                shapes::arrow(&self.painter, center, end);
            }
            Markup::Line(end_x, end_y) => {
                let end = egui::Pos2 {
                    x: self.inner_rect.min.x + self.spacing.x * (end_x as f32),
                    y: self.inner_rect.min.y + self.spacing.y * (end_y as f32),
                };
                shapes::line(&self.painter, center, end);
            }
            Markup::Label(s) => {
                shapes::label(&self.painter, &s, center, self.stone_radius);
            }
        }
    }

    pub fn handle_click(
        &self,
        ui: &mut Ui,
        board: &mut Board,
        tool: &mut UiTool,
        turn: &mut Stone,
    ) -> Action {
        if !self.response.clicked() {
            return Action::NoOp;
        }

        let (w, h) = board.size();

        let op_pos = ui.input(|i| i.pointer.interact_pos());
        if op_pos.is_none() {
            #[cfg(debug_assertions)]
            crate::log("pointer interact pos is None");

            return Action::NoOp;
        }

        let p = op_pos.unwrap();

        let (x, y) = (
            (((p.x - self.inner_rect.min.x) / self.spacing.x).round() as usize).min(w - 1),
            (((p.y - self.inner_rect.min.y) / self.spacing.y).round() as usize).min(h - 1),
        );

        match tool.tool {
            ToolType::Play => {
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

            ToolType::Circle => {
                if board.set_markup(x, y, Markup::Circle) {
                    return Action::Circle(vec![(x, y)]);
                }
            }
            ToolType::Cross => {
                if board.set_markup(x, y, Markup::Cross) {
                    return Action::Cross(vec![(x, y)]);
                }
            }
            ToolType::Square => {
                if board.set_markup(x, y, Markup::Square) {
                    return Action::Square(vec![(x, y)]);
                }
            }
            ToolType::Triangle => {
                if board.set_markup(x, y, Markup::Triangle) {
                    return Action::Triangle(vec![(x, y)]);
                }
            }
            ToolType::Dim => {
                if board.set_markup(x, y, Markup::Dim) {
                    return Action::Dim(vec![(x, y)]);
                }
            }
            ToolType::Arrow => {
                if let Some((sx, sy)) = tool.base {
                    tool.base = None;

                    if board.set_markup(sx, sy, Markup::Arrow(x, y)) {
                        return Action::Arrow(vec![[(sx, sy), (x, y)]]);
                    }
                } else {
                    tool.base = Some((x, y));
                }
            }
            ToolType::Line => {
                if let Some((sx, sy)) = tool.base {
                    tool.base = None;

                    if board.set_markup(sx, sy, Markup::Line(x, y)) {
                        return Action::Line(vec![[(sx, sy), (x, y)]]);
                    }
                } else {
                    tool.base = Some((x, y));
                }
            }
            ToolType::Number => {
                let text = format!("{}", tool.number);
                if board.set_markup(x, y, Markup::Label(text.clone())) {
                    tool.number += 1;
                    return Action::Label(vec![(x, y, text)]);
                }
            }
            ToolType::Letter => {
                let text = format!("{}", tool.letter);
                if board.set_markup(x, y, Markup::Label(text.clone())) {
                    tool.letter = crate::util::next_letter(tool.letter);
                    return Action::Label(vec![(x, y, text)]);
                }
            }
        }

        Action::NoOp
    }
}
