use macroquad::prelude::*;
mod bodies;
use bodies::PhysicsBox;
mod collider;
mod physics;
use physics::update;
mod renderer;
use crate::collider::collision_check;
mod sandbox;

use crate::physics::apply_force_from_point;
use crate::renderer::{draw_body, preview_body};
use crate::sandbox::tools::rect_tool;

#[macroquad::main("rust sim")]
async fn main() {
    let mut bodies: Vec<PhysicsBox> = Vec::new();

    let mut drag_start: Option<Vec2> = None;
    let mut drag_end: Option<Vec2> = None;
    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        let mouse = vec2(mouse_position().0, mouse_position().1);
        // use rectangle tool
        if is_mouse_button_pressed(MouseButton::Right) {
            drag_start = Some(mouse);
        }

        if is_mouse_button_down(MouseButton::Right) {
            match drag_start {
                Some(x) => preview_body(x, mouse),
                None => return,
            }
        }

        if is_mouse_button_released(MouseButton::Right) {
            drag_end = Some(mouse);
            match (drag_start, drag_end) {
                (Some(x), Some(y)) => {
                    bodies.push(rect_tool(x, y));
                }
                _ => {
                    println!("Failed to create rect");
                }
            }
        }
        // Repelling force when left click is held
        if is_mouse_button_down(MouseButton::Left) {
            for body in bodies.iter_mut() {
                apply_force_from_point(body, mouse.x, mouse.y, dt);
            }
        }

        collision_check(&mut bodies);
        update(&mut bodies, dt);
        for body in bodies.iter() {
            draw_body(body);
        }

        next_frame().await
    }
}
