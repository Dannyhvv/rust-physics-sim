use crate::Vec2;
use macroquad::color::Color;

pub enum Shape {
    Box { w: f32, h: f32 },
    Ball { r: f32 },
}

pub struct Body {
    pub pos: Vec2,
    pub shape: Shape,
    pub m: f32,
    pub vel: Vec2,
    pub rot: f32,
    pub ang_vel: f32,
    pub force: Vec2,
    pub torque: f32,
    pub moi: f32,
    pub res: f32,
    pub friction: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub is_static: bool,
    pub can_collide: bool,
    pub color: Color,
}

impl Body {
    pub fn new_box(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        m: f32,
        rot: f32,
        res: f32,
        friction: f32,
        is_static: bool,
        can_collide: bool,
        color: Color,
    ) -> Self {
        Body {
            pos: Vec2::new(x, y),
            shape: Shape::Box { w, h },
            m,
            vel: Vec2::ZERO,
            rot,
            ang_vel: 0.0,
            force: Vec2::ZERO,
            torque: 0.0,
            moi: (1.0 / 12.0) * m * (w * w + h * h),
            res,
            friction,
            linear_damping: 0.85,
            angular_damping: 1.2,
            is_static,
            can_collide,
            color,
        }
    }

    pub fn new_ball(
        x: f32,
        y: f32,
        r: f32,
        m: f32,
        rot: f32,
        res: f32,
        friction: f32,
        is_static: bool,
        can_collide: bool,
        color: Color,
    ) -> Self {
        Body {
            pos: Vec2::new(x, y),
            shape: Shape::Ball { r },
            m,
            vel: Vec2::ZERO,
            rot,
            ang_vel: 0.0,
            force: Vec2::ZERO,
            torque: 0.0,
            moi: 0.5 * m * r * r,
            res,
            friction,
            linear_damping: 0.85,
            angular_damping: 1.2,
            is_static,
            can_collide,
            color,
        }
    }
}
