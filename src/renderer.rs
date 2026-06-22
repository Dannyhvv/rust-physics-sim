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

pub fn preview_box(corner1: Vec2, corner2: Vec2, camera: &Camera2D) {
    let s1 = camera.world_to_screen(corner1);
    let s2 = camera.world_to_screen(corner2);
    set_default_camera();
    draw_rectangle_lines(
        s1.x.min(s2.x),
        s1.y.min(s2.y),
        (s2.x - s1.x).abs(),
        (s2.y - s1.y).abs(),
        2.0,
        GREEN,
    );
    set_camera(camera);
}

pub fn preview_ball(center: Vec2, mouse: Vec2, camera: &Camera2D) {
    let sc = camera.world_to_screen(center);
    let sm = camera.world_to_screen(mouse);
    let r = sc.distance(sm);
    set_default_camera();
    draw_poly_lines(sc.x, sc.y, 64, r, 0., 2.0, GREEN);
    set_camera(camera);
}

pub fn draw_mouse_line(pos: Vec2, mouse: Vec2, enabled: bool, camera: &Camera2D) {
    if !enabled {
        return;
    }
    let sp = camera.world_to_screen(pos);
    let sm = camera.world_to_screen(mouse);
    set_default_camera();
    draw_line(sp.x, sp.y, sm.x, sm.y, 2.0, WHITE);
    set_camera(camera);
}
