use std::f32::consts::FRAC_PI_4;
use std::f32::consts::SQRT_2;
const SQRT_3: f32 = 1.73205078;

use super::board::Computed;
use super::BoardStyle;

use eframe::egui;
use egui::epaint;
use egui::pos2;
use egui::Pos2;
use egui::Ui;
use epaint::Color32;
use epaint::Shape;

/// Stroke a square within a circle
pub fn find_square(center: Pos2, r: f32, style: &BoardStyle) -> Shape {
    let top_left = pos2(center.x - 0.5 * r * SQRT_2, center.y - 0.5 * r * SQRT_2);
    let bottom_right = pos2(center.x + 0.5 * r * SQRT_2, center.y + 0.5 * r * SQRT_2);

    let rect = egui::Rect {
        min: top_left,
        max: bottom_right,
    };

    return Shape::rect_stroke(rect, egui::Rounding::none(), style.marker_stroke);
}

/// Stroke a triangle within a circle
pub fn find_triangle(center: Pos2, r: f32, style: &BoardStyle) -> Shape {
    let top = pos2(center.x, center.y - r);
    let left = pos2(center.x - r * 0.5 * SQRT_3, center.y + r * 0.5);
    let right = pos2(center.x + r * 0.5 * SQRT_3, center.y + r * 0.5);

    return Shape::convex_polygon(
        vec![top, left, right],
        Color32::TRANSPARENT,
        style.marker_stroke,
    );
}

/// Stroke a cross or 'x' shape within a circle
pub fn find_cross(center: Pos2, r: f32, style: &BoardStyle) -> Shape {
    let top_left = pos2(center.x - 0.5 * r * SQRT_2, center.y - 0.5 * r * SQRT_2);
    let top_right = pos2(center.x + 0.5 * r * SQRT_2, center.y - 0.5 * r * SQRT_2);
    let bottom_right = pos2(center.x + 0.5 * r * SQRT_2, center.y + 0.5 * r * SQRT_2);
    let bottom_left = pos2(center.x - 0.5 * r * SQRT_2, center.y + 0.5 * r * SQRT_2);

    return Shape::Vec(vec![
        Shape::line_segment([top_left, bottom_right], style.marker_stroke),
        Shape::line_segment([top_right, bottom_left], style.marker_stroke),
    ]);
}

/// Find a character that fits within the circle
pub fn find_char(ui: &mut Ui, center: Pos2, r: f32, c: char, style: &BoardStyle) -> Shape {
    Shape::text(
        &ui.fonts(),
        center,
        egui::Align2::CENTER_CENTER,
        c,
        egui::FontId::monospace(r * 2.0),
        style.marker_stroke.color,
    )
}

/// Fill a triangle at the end of an arrow
pub fn find_arrow(start: Pos2, end: Pos2, c: &Computed, style: &BoardStyle) -> Shape {
    let angle = f32::atan2(end.y - start.y, end.x - start.x);

    let p1 = pos2(
        end.x - c.arrow_size * f32::cos(angle + FRAC_PI_4),
        end.y - c.arrow_size * f32::sin(angle + FRAC_PI_4),
    );
    let p2 = pos2(
        end.x - c.arrow_size * f32::cos(angle - FRAC_PI_4),
        end.y - c.arrow_size * f32::sin(angle - FRAC_PI_4),
    );

    return Shape::convex_polygon(vec![p1, end, p2], style.marker_stroke.color, style.marker_stroke);
}
