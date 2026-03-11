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
