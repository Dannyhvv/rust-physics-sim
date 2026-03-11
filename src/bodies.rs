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
    pub mass_offset: Vec2,
    pub is_static: bool,
    pub can_collide: bool,
}

impl PhysicsBox {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        m: f32,
        rot: f32,
        is_static: bool,
        can_collide: bool,
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
            res: 0.8,
            mass_offset: Vec2::new(0.0, 0.0),
            is_static: is_static,
            can_collide: can_collide,
        }
    }
}
