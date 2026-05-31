use crate::Vec2;
use crate::bodies::PhysicsBox;

pub fn rect_tool(corner1: Vec2, corner2: Vec2) -> PhysicsBox {
    let w = corner2.x - corner1.x;
    let h = corner2.y - corner1.y;
    return PhysicsBox::new(
        corner1.x + w / 2.0,
        corner1.y + h / 2.0,
        w,
        h,
        20.0,
        0.0,
        false,
        true,
    );
}
