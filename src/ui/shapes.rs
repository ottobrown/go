use std::f32::consts::SQRT_2;
const SQRT_3: f32 = 1.73205078;

use super::BoardStyle;

use eframe::egui;
use egui::epaint;
use egui::pos2;
use egui::Pos2;
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

    return Shape::rect_stroke(
        rect,
        egui::Rounding::none(),
        egui::Stroke::new(style.marker_stroke, Color32::RED),
    );
}

/// Stroke a triangle withing a circle
pub fn find_triangle(center: Pos2, r: f32, style: &BoardStyle) -> Shape {
    let top = pos2(center.x, center.y - r);
    let left = pos2(center.x - r*0.5*SQRT_3, center.y + r*0.5);
    let right = pos2(center.x + r*0.5*SQRT_3, center.y + r*0.5);

    return Shape::convex_polygon(
        vec![top, left, right],
        Color32::TRANSPARENT,
        egui::Stroke::new(style.marker_stroke, Color32::RED),
    );
}
