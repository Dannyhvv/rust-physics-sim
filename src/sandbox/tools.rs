use crate::Vec2;
use crate::bodies::Body;
use crate::physics::*;
use crate::renderer::*;
use ::rand::prelude::IndexedRandom;
use ::rand::rng;
use macroquad::color::*;
use macroquad::prelude::*;

pub enum ActiveTool {
    Drag,
    Rectangle,
    Circle,
}

pub struct RectTool {
    pub drag_start: Option<Vec2>,

    // settings
    pub density: f32,
    pub restitution: f32,
    pub random_colors: bool,
    pub color: Color,
    pub is_static: bool,
    pub can_collide: bool,
}

impl RectTool {
    pub fn new() -> Self {
        Self {
            drag_start: None,

            density: 20.0,
            restitution: 0.8,
            random_colors: true,
            color: WHITE,
            is_static: false,
            can_collide: true,
        }
    }

    pub fn update(&mut self, mouse: Vec2, bodies: &mut Vec<Body>) {
        // Preview rectangle position with outline
        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_start = Some(mouse);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                preview_box(start, mouse);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                bodies.push(self.create_box(start, mouse));
            }

            self.drag_start = None;
        }
    }

    pub fn create_box(&self, corner1: Vec2, corner2: Vec2) -> Body {
        let min_x = corner1.x.min(corner2.x);
        let min_y = corner1.y.min(corner2.y);

        let w = (corner2.x - corner1.x).abs();
        let h = (corner2.y - corner1.y).abs();

        let colors = [RED, GREEN, BLUE, PURPLE, ORANGE];

        let color = if self.random_colors {
            *colors.choose(&mut rng()).unwrap()
        } else {
            self.color
        };

        Body::new_box(
            min_x + w / 2.0,
            min_y + h / 2.0,
            w,
            h,
            self.density,
            0.0,
            self.restitution,
            self.is_static,
            self.can_collide,
            color,
        )
    }
}
pub struct BallTool {
    pub drag_start: Option<Vec2>,

    pub density: f32,
    pub restitution: f32,
    pub random_colors: bool,
    pub color: Color,
    pub is_static: bool,
    pub can_collide: bool,
}
impl BallTool {
    pub fn new() -> Self {
        Self {
            drag_start: None,

            density: 20.0,
            restitution: 0.8,
            random_colors: true,
            color: WHITE,
            is_static: false,
            can_collide: true,
        }
    }
    pub fn update(&mut self, mouse: Vec2, bodies: &mut Vec<Body>) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_start = Some(mouse);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                preview_ball(start, mouse);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                bodies.push(self.create_ball(start, mouse));
            }

            self.drag_start = None;
        }
    }

    pub fn create_ball(&self, center: Vec2, mouse: Vec2) -> Body {
        let r = center.distance(mouse);

        let colors = [RED, GREEN, BLUE, PURPLE, ORANGE];

        let color = if self.random_colors {
            *colors.choose(&mut rng()).unwrap()
        } else {
            self.color
        };

        Body::new_ball(
            center.x,
            center.y,
            r,
            self.density,
            0.0,
            self.restitution,
            self.is_static,
            self.can_collide,
            color,
        )
    }
}

pub struct DragTool {
    pub selected: Option<usize>,
    pub strength: f32,
    pub damping: f32,
    pub draw_line: bool,
    pub line_target: Vec2,
}
impl DragTool {
    pub fn new() -> Self {
        Self {
            selected: None,
            strength: 50.0,
            damping: 15.0,
            draw_line: false,
            line_target: (Vec2 { x: 0.0, y: 0.0 }),
        }
    }

    pub fn update(&mut self, mouse: Vec2, bodies: &mut Vec<Body>, dt: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.selected = get_nearest_body(mouse, bodies);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(index) = self.selected {
                apply_spring_to_point(&mut bodies[index], mouse, self.strength, self.damping);

                // Update line info
                self.draw_line = true;
                self.line_target = Vec2 {
                    x: bodies[index].pos.x,
                    y: bodies[index].pos.y,
                };
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.selected = None;
            self.draw_line = false;
        }
    }
}

// Helper functions
pub fn get_nearest_body(mouse: Vec2, bodies: &[Body]) -> Option<usize> {
    if bodies.is_empty() {
        return None;
    }

    let mut nearest_distance = mouse.distance(bodies[0].pos);
    let mut candidate_index = 0;

    for i in 1..bodies.len() {
        let candidate_distance = mouse.distance(bodies[i].pos);

        if candidate_distance < nearest_distance {
            nearest_distance = candidate_distance;
            candidate_index = i;
        }
    }

    Some(candidate_index)
}
