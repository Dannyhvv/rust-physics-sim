use crate::bodies::{Body, Shape};
use macroquad::prelude::*;

pub fn draw_body(body: &Body) {
    match body.shape {
        Shape::Box { w, h } => {
            draw_rectangle_ex(
                body.pos.x,
                body.pos.y,
                w,
                h,
                DrawRectangleParams {
                    offset: Vec2::new(0.5, 0.5),
                    rotation: body.rot,
                    color: body.color,
                },
            );
        }

        Shape::Ball { r } => {
            draw_poly(body.pos.x, body.pos.y, 255, r, 0., body.color);
        }
    }
}

pub fn preview_box(corner1: Vec2, corner2: Vec2) {
    draw_rectangle_lines(
        corner1.x,
        corner1.y,
        corner2.x - corner1.x,
        corner2.y - corner1.y,
        2.0,
        GREEN,
    );
}
pub fn preview_ball(center: Vec2, mouse: Vec2) {
    let r = center.distance(mouse);
    draw_poly_lines(center.x, center.y, 255, r, 0., 2.0, GREEN);
}
pub fn draw_mouse_line(pos: Vec2, mouse: Vec2, enabled: bool) {
    if !enabled {
        return;
    };
    draw_line(pos.x, pos.y, mouse.x, mouse.y, 2.0, WHITE);
}
