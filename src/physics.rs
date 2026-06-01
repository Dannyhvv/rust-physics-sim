use crate::{Vec2, bodies::PhysicsBox};

pub fn apply_force(body: &mut PhysicsBox, force: Vec2, dt: f32) {
    let a = force / body.m;
    body.vel += a * dt;
}

pub fn apply_force_from_point(
    body: &mut PhysicsBox,
    x_pos: f32,
    y_pos: f32,
    strength: f32,
    damping: f32,
    dt: f32,
) {
    let temp = Vec2::new(body.pos.x - x_pos, body.pos.y - y_pos);

    let distance = temp.length();
    if distance < f32::EPSILON {
        return;
    }
    let direction = temp / distance;

    let force = direction * -distance * strength;

    let damping = body.vel * -damping;

    apply_force(body, force + damping, dt);
}

pub fn update(bodies: &mut Vec<PhysicsBox>, dt: f32) {
    for body in bodies.iter_mut() {
        if body.is_static {
            continue;
        }
        body.vel *= 1.0 - (0.85 * dt);
        body.ang_vel *= 1.0 - (2.55 * dt);

        if body.ang_vel.abs() < 0.05 {
            body.ang_vel = 0.0;
        }

        body.pos += body.vel * dt;
        body.rot += body.ang_vel * dt;
    }
}
