use crate::bodies::PhysicsBox;
use macroquad::prelude::*;

pub fn draw_body(body: &PhysicsBox) {
    draw_rectangle_ex(
        body.pos.x,
        body.pos.y,
        body.w,
        body.h,
        DrawRectangleParams {
            offset: (Vec2 { x: (0.5), y: (0.5) }),
            rotation: (body.rot),
            color: (BLUE),
        },
    )
}
pub fn preview_body(corner1: Vec2, corner2: Vec2) {
    draw_rectangle_lines(
        corner1.x,
        corner1.y,
        corner2.x - corner1.x,
        corner2.y - corner1.y,
        2.0,
        GREEN,
    );
}
