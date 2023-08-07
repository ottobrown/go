use eframe::egui;
use egui::{pos2, Color32, Painter, Pos2};

pub fn circle(p: &Painter, center: Pos2, stone_radius: f32) {
    p.circle_stroke(center, 0.75 * stone_radius, (2.0, Color32::RED));
}

pub fn cross(p: &Painter, center: Pos2, stone_radius: f32) {
    let points1 = [
        pos2(center.x - stone_radius, center.y - stone_radius),
        pos2(center.x + stone_radius, center.y + stone_radius),
    ];
    let points2 = [
        pos2(center.x + stone_radius, center.y - stone_radius),
        pos2(center.x - stone_radius, center.y + stone_radius),
    ];
    p.line_segment(points1, (2.0, Color32::RED));
    p.line_segment(points2, (2.0, Color32::RED));
}

pub fn square(p: &Painter, center: Pos2, stone_radius: f32) {
    let rect = egui::Rect {
        min: pos2(center.x - stone_radius, center.y - stone_radius),
        max: pos2(center.x + stone_radius, center.y + stone_radius),
    };

    p.rect_stroke(rect, 0.0, (2.0, Color32::RED));
}
