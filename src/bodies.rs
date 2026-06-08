use macroquad::color::Color;

use crate::Vec2;
pub struct PhysicsBox {
    pub pos: Vec2,
    pub w: f32,
    pub h: f32,
    pub m: f32,
    pub vel: Vec2,
    pub rot: f32,
    pub ang_vel: f32,
    pub moi: f32,
    pub res: f32,
    pub is_static: bool,
    pub can_collide: bool,
    pub color: Color,
}

impl PhysicsBox {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        m: f32,
        rot: f32,
        res: f32,
        is_static: bool,
        can_collide: bool,
        color: Color,
    ) -> Self {
        PhysicsBox {
            pos: Vec2::new(x, y),
            w: w,
            h: h,
            m: m,
            vel: Vec2::new(0.0, 0.0),
            rot: rot,
            ang_vel: 0.0,
            moi: (1.0 / 12.0) * m * (w * w + h * h),
            res: res,
            is_static: is_static,
            can_collide: can_collide,
            color: color,
        }
    }
}
