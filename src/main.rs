use macroquad::prelude::*;
mod bodies;
use bodies::PhysicsBox;
mod collider;
mod physics;
use physics::update;
mod renderer;
use crate::collider::collision_check;
mod sandbox;

use crate::renderer::{draw_body, draw_mouse_line};
use crate::sandbox::tools::{ActiveTool, DragTool, RectTool};
use crate::sandbox::ui::*;
use macroquad::ui::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut bodies: Vec<PhysicsBox> = Vec::new();
    let mut active_tool = ActiveTool::Drag;
    let mut drag_tool = DragTool::new();
    let mut rect_tool = RectTool::new();

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        let mouse = vec2(mouse_position().0, mouse_position().1);

        let mouse_over_ui = root_ui().is_mouse_over(mouse);
        if !mouse_over_ui {
            match active_tool {
                ActiveTool::Drag => {
                    drag_tool.update(mouse, &mut bodies, dt);
                }

                ActiveTool::Rectangle => {
                    rect_tool.update(mouse, &mut bodies);
                }
            }
        }

        update(&mut bodies, dt);
        collision_check(&mut bodies);
        for body in bodies.iter() {
            draw_body(body);
        }
        // Draw UI
        draw_ui(&mut active_tool, &mut drag_tool, &mut rect_tool);
        draw_mouse_line(drag_tool.line_target, mouse, drag_tool.draw_line);

        next_frame().await
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "rust sim".to_owned(),
        window_width: 800,
        window_height: 600,
        sample_count: 8,
        ..Default::default()
    }
}
