use macroquad::prelude::*;
mod bodies;
use bodies::Body;
mod collider;
mod physics;
use physics::*;
mod renderer;
use crate::collider::collision_check;
mod sandbox;

use crate::renderer::{draw_body, draw_mouse_line};
use crate::sandbox::tools::{ActiveTool, BallTool, DragTool, RectTool};
use crate::sandbox::ui::*;
use macroquad::ui::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World {
        gravity: vec2(0.0, 500.0),
        paused: false,
        solver_iterations: 8,
    };
    let mut bodies: Vec<Body> = Vec::new();
    let mut active_tool = ActiveTool::Drag;
    let mut drag_tool = DragTool::new();
    let mut rect_tool = RectTool::new();
    let mut ball_tool = BallTool::new();

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
                ActiveTool::Circle => {
                    ball_tool.update(mouse, &mut bodies);
                }
            }
        }

        if !world.paused {
            for body in bodies.iter_mut() {
                physics::apply_gravity(body, world.gravity);
            }

            update(&mut bodies, dt);

            for _ in 0..world.solver_iterations {
                collision_check(&mut bodies);
            }
        }
        for body in bodies.iter() {
            draw_body(body);
        }
        // Draw UI
        let reset_sim = draw_ui(
            &mut world,
            &mut active_tool,
            &mut drag_tool,
            &mut rect_tool,
            &mut ball_tool,
            dt,
        );

        if reset_sim {
            bodies.clear();
        }

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
