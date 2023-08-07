use eframe::egui;
use egui::{pos2, Color32, Painter, Pos2, Vec2};

use std::f32::consts::FRAC_PI_4;

pub fn circle(p: &Painter, center: Pos2, stone_radius: f32) {
    let r = 0.75 * stone_radius;
    p.circle_stroke(center, r, (2.0, Color32::RED));
}

pub fn cross(p: &Painter, center: Pos2, stone_radius: f32) {
    let r = 0.75 * stone_radius;
    let points1 = [
        pos2(center.x - r, center.y - r),
        pos2(center.x + r, center.y + r),
    ];
    let points2 = [
        pos2(center.x + r, center.y - r),
        pos2(center.x - r, center.y + r),
    ];
    p.line_segment(points1, (2.0, Color32::RED));
    p.line_segment(points2, (2.0, Color32::RED));
}

pub fn square(p: &Painter, center: Pos2, stone_radius: f32) {
    let r = 0.75 * stone_radius;
    let rect = egui::Rect {
        min: pos2(center.x - r, center.y - r),
        max: pos2(center.x + r, center.y + r),
    };

    p.rect_stroke(rect, 0.0, (2.0, Color32::RED));
}

pub fn triangle(p: &Painter, center: Pos2, stone_radius: f32) {
    let r = 0.75 * stone_radius;

    let top = pos2(center.x, center.y - r);
    let left = pos2(center.x - r, center.y + r);
    let right = pos2(center.x + r, center.y + r);

    p.line_segment([top, left], (2.0, Color32::RED));
    p.line_segment([top, right], (2.0, Color32::RED));
    p.line_segment([left, right], (2.0, Color32::RED));
}

pub fn dim(p: &Painter, center: Pos2, spacing: Vec2) {
    let color = Color32::from_rgba_premultiplied(100, 100, 100, 100);

    let rect = egui::Rect {
        min: pos2(center.x - 0.5 * spacing.x, center.y - 0.5 * spacing.y),
        max: pos2(center.x + 0.5 * spacing.x, center.y + 0.5 * spacing.y),
    };

    p.rect_filled(rect, 0.0, color);
}

pub fn line(p: &Painter, start: Pos2, end: Pos2) {
    p.line_segment([start, end], (4.0, Color32::RED));
}

pub fn arrow(p: &Painter, start: Pos2, end: Pos2) {
    p.line_segment([start, end], (4.0, Color32::RED));
    let angle = egui::vec2(-end.x + start.x, -end.y + start.y).angle();

    let points = vec![
        end,
        end + 15.0 * Vec2::angled(angle + FRAC_PI_4),
        end + 15.0 * Vec2::angled(angle - FRAC_PI_4),
    ];

    let triangle = egui::Shape::convex_polygon(points, Color32::RED, (2.0, Color32::RED));

    p.add(triangle);
}

pub fn label(p: &Painter, s: &str, center: Pos2, stone_radius: f32) {
    let font = egui::FontId {
        size: 2.0 * stone_radius,
        family: egui::FontFamily::Monospace,
    };
    p.text(center, egui::Align2::CENTER_CENTER, s, font, Color32::RED);
}
