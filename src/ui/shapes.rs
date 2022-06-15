use std::f32::consts::SQRT_2;

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
    todo!()
}
