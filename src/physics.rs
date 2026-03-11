use crate::{Vec2, bodies::PhysicsBox};

pub fn apply_force(body: &mut PhysicsBox, force: Vec2, dt: f32) {
    let a = force / body.m;
    body.vel += a * dt;
}

pub fn apply_force_from_point(body: &mut PhysicsBox, x_pos: f32, y_pos: f32, dt: f32) {
    let temp = Vec2::new(body.pos.x - x_pos, body.pos.y - y_pos);
    let distance = temp.length();
    let direction = temp.normalize();

    let force = direction * (50000.0 / distance);

    apply_force(body, force, dt);
}

// Main update loop
pub fn update(bodies: &mut Vec<PhysicsBox>, dt: f32) {
    for body in bodies.iter_mut() {
        if body.is_static {
            continue;
        }
        //drag
        apply_force(body, -body.vel * 0.85, dt);
        body.ang_vel += -(body.ang_vel * 0.85) * dt;

        body.pos += body.vel * dt;
        body.rot += body.ang_vel * dt;
    }
}
