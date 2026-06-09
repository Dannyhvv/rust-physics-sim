use crate::{Vec2, bodies::Body};

pub struct World {
    pub gravity: Vec2,
    pub paused: bool,
    pub solver_iterations: u32,
}

pub fn apply_force(body: &mut Body, force: Vec2) {
    body.force += force;
}

pub fn apply_torque(body: &mut Body, torque: f32) {
    body.torque += torque;
}

pub fn apply_force_at_point(body: &mut Body, force: Vec2, point: Vec2) {
    body.force += force;

    let r = point - body.pos;

    let torque = r.x * force.y - r.y * force.x;

    body.torque += torque;
}

pub fn apply_gravity(body: &mut Body, gravity: Vec2) {
    if body.is_static {
        return;
    }

    body.force += gravity * body.m;
}

pub fn apply_spring_to_point(body: &mut Body, point: Vec2, strength: f32, damping: f32) {
    let offset = body.pos - point;

    let distance = offset.length();

    if distance < f32::EPSILON {
        return;
    }

    let direction = offset / distance;

    let spring_force = direction * (-distance * strength);

    let damping_force = body.vel * -damping;

    apply_force(body, spring_force + damping_force);
}

pub fn apply_damping(body: &mut Body, dt: f32) {
    body.vel *= 1.0 - body.linear_damping * dt;
    body.ang_vel *= 1.0 - body.angular_damping * dt;
}

pub fn update(bodies: &mut [Body], dt: f32) {
    for body in bodies.iter_mut() {
        if body.is_static {
            continue;
        }

        let acceleration = body.force / body.m;
        let angular_acceleration = body.torque / body.moi;

        body.vel += acceleration * dt;
        body.ang_vel += angular_acceleration * dt;

        apply_damping(body, dt);

        body.pos += body.vel * dt;
        body.rot += body.ang_vel * dt;

        body.force = Vec2::ZERO;
        body.torque = 0.0;
    }
}
