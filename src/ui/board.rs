use crate::Board;
use eframe::egui;
use egui::epaint;
use egui::Ui;
use egui::{pos2, vec2};
use epaint::{Color32, Shape, Stroke};

#[derive(Clone, Copy)]
/// Specifies display of board
pub struct BoardStyle {
    pub background_color: Color32,
    /// Amount of empty space on each side of the board
    /// Expressed as a proportion of the total board width/height.
    pub padding: f32,
    /// In egui screen units
    pub line_thickness: f32,
    /// As a proportion of the width/height of a board square (whichever is smaller)
    pub stone_radius: f32,
}

impl Default for BoardStyle {
    fn default() -> Self {
        Self {
            // 0xDEB887, burlywood in CSS
            background_color: Color32::from_rgb(0xDE, 0xB8, 0x87),
            padding: 0.05,
            line_thickness: 3.0,
            stone_radius: 0.4,
        }
    }
}

/// Exact values for board
pub struct Computed {
    pub outer_rect: egui::Rect,
    pub inner_rect: egui::Rect,
    pub padding: egui::Vec2,
    pub spacing: egui::Vec2,
    pub stone_radius: f32,
}
impl Computed {
    /// Initialize Self when no values are known
    /// i.e. when app is first opened
    pub fn blank() -> Self {
        Self {
            outer_rect: egui::Rect::NOTHING,
            inner_rect: egui::Rect::NOTHING,
            padding: egui::Vec2::ZERO,
            spacing: egui::Vec2::ZERO,
            stone_radius: 0.0,
        }
    }

    pub fn compute(w: usize, h: usize, response: egui::Response, style: &BoardStyle) -> Self {
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

        let min = {
            if spacing.x < spacing.y {
                spacing.x
            } else {
                spacing.y
            }
        };
        let stone_radius = min * style.stone_radius;

        return Self {
            outer_rect: response.rect,
            inner_rect: inner_rect,
            padding: padding,
            spacing: spacing,
            stone_radius: stone_radius,
        };
    }

    /// Get exact screen position of point on the board
    pub fn get_pos(&self, x: usize, y: usize) -> egui::Pos2 {
        let x_pos = self.inner_rect.min.x + self.spacing.x * (x as f32);
        let y_pos = self.inner_rect.min.y + self.spacing.y * (y as f32);

        return pos2(x_pos, y_pos);
    }
}

pub fn render_board(
    ui: &mut Ui,
    board: &Board,
    style: &BoardStyle,
    size: egui::Vec2,
    c: &mut Computed,
) {
    let (response, painter) = ui.allocate_painter(vec2(size.x, size.y), egui::Sense::drag());
    let (w, h) = (board.width(), board.height());

    *c = Computed::compute(w, h, response, style);

    let mut shapes = Vec::new();

    // Draw background color
    let background = Shape::rect_filled(
        egui::Rect::EVERYTHING,
        egui::Rounding::none(),
        style.background_color,
    );
    shapes.push(background);

    let outline = Shape::rect_stroke(
        c.inner_rect,
        egui::Rounding::none(),
        egui::Stroke::new(style.line_thickness, Color32::BLACK),
    );
    shapes.push(outline);

    // draw vertical lines
    for x in 0..w {
        let x_pos = c.inner_rect.min.x + c.spacing.x * (x as f32);
        let start_y = c.inner_rect.min.y;
        let end_y = c.inner_rect.max.y;

        let line = Shape::line_segment(
            [pos2(x_pos, start_y), pos2(x_pos, end_y)],
            Stroke::from((style.line_thickness, Color32::BLACK)),
        );

        shapes.push(line);
    }

    // draw horizontal lines
    for y in 0..h {
        let y_pos = c.inner_rect.min.y + c.spacing.y * (y as f32);
        let start_x = c.inner_rect.min.x;
        let end_x = c.inner_rect.max.x;

        let line = Shape::line_segment(
            [pos2(start_x, y_pos), pos2(end_x, y_pos)],
            Stroke::from((style.line_thickness, Color32::BLACK)),
        );

        shapes.push(line)
    }

    painter.extend(shapes);
}
