use macroquad::prelude::*;
mod bodies;
use bodies::PhysicsBox;
mod collider;
mod physics;
use physics::update;
mod renderer;
use crate::collider::collision_check;

use crate::physics::apply_force_from_point;
use crate::renderer::draw_body;

#[macroquad::main("rust sim")]
async fn main() {
    let mut bodies: Vec<PhysicsBox> = Vec::new();
    bodies.push(PhysicsBox::new(
        200.0, 200.0, 20.0, 60.0, 20.0, 1.0, false, true,
    ));
    bodies.push(PhysicsBox::new(
        250.0, 200.0, 20.0, 20.0, 20.0, 1.0, false, true,
    ));
    bodies.push(PhysicsBox::new(
        300.0, 200.0, 20.0, 69.0, 20.0, 1.0, false, true,
    ));

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        if is_mouse_button_down(MouseButton::Left) {
            for body in bodies.iter_mut() {
                apply_force_from_point(body, mouse_position().0, mouse_position().1, dt);
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
