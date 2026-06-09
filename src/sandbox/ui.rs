use crate::physics::World;
use crate::sandbox::tools::{ActiveTool, BallTool, DragTool, RectTool};
use crate::vec2;
use macroquad::prelude::*;
use macroquad::ui::*;

pub fn draw_ui(
    world: &mut World,
    active_tool: &mut ActiveTool,
    drag_tool: &mut DragTool,
    rect_tool: &mut RectTool,
    ball_tool: &mut BallTool,
    dt: f32,
) -> bool {
    let mut reset_sim = false;

    root_ui().window(
        hash!("main_panel"),
        vec2(10.0, 10.0),
        vec2(300.0, 450.0),
        |ui| {
            ui.label(None, "Physics Sandbox");

            ui.separator();

            // Tool selector

            ui.label(None, "TOOLS");

            let drag_name = if matches!(*active_tool, ActiveTool::Drag) {
                "▶ Drag"
            } else {
                "Drag"
            };

            let rect_name = if matches!(*active_tool, ActiveTool::Rectangle) {
                "▶ Rect"
            } else {
                "Rect"
            };

            let circle_name = if matches!(*active_tool, ActiveTool::Circle) {
                "▶ Circle"
            } else {
                "Circle"
            };

            if ui.button(None, drag_name) {
                *active_tool = ActiveTool::Drag;
            }

            ui.same_line(0.0);

            if ui.button(None, rect_name) {
                *active_tool = ActiveTool::Rectangle;
            }

            ui.same_line(0.0);

            if ui.button(None, circle_name) {
                *active_tool = ActiveTool::Circle;
            }

            ui.separator();

            // Tools

            ui.label(None, "Tool Settings");

            match active_tool {
                ActiveTool::Drag => {
                    ui.label(None, "Drag Tool");

                    ui.slider(hash!(), "Strength", 0.0..100.0, &mut drag_tool.strength);

                    ui.slider(hash!(), "Damping", 0.0..50.0, &mut drag_tool.damping);
                }

                ActiveTool::Rectangle => {
                    ui.label(None, "Rectangle Tool");

                    ui.slider(hash!(), "Density", 1.0..100.0, &mut rect_tool.density);

                    ui.slider(hash!(), "Restitution", 0.0..1.0, &mut rect_tool.restitution);

                    ui.checkbox(hash!(), "Static", &mut rect_tool.is_static);

                    ui.checkbox(hash!(), "Collide", &mut rect_tool.can_collide);

                    ui.checkbox(hash!(), "Random Colors", &mut rect_tool.random_colors);
                }

                ActiveTool::Circle => {
                    ui.label(None, "Circle Tool");

                    ui.slider(hash!(), "Density", 1.0..100.0, &mut ball_tool.density);

                    ui.slider(hash!(), "Restitution", 0.0..1.0, &mut ball_tool.restitution);

                    ui.checkbox(hash!(), "Static", &mut ball_tool.is_static);

                    ui.checkbox(hash!(), "Collide", &mut ball_tool.can_collide);

                    ui.checkbox(hash!(), "Random Colors", &mut ball_tool.random_colors);
                }
            }

            ui.separator();

            // World

            ui.label(None, "WORLD");

            ui.checkbox(hash!(), "Paused", &mut world.paused);

            ui.slider(hash!(), "Gravity X", -1000.0..1000.0, &mut world.gravity.x);

            ui.slider(hash!(), "Gravity Y", -1000.0..1000.0, &mut world.gravity.y);

            let mut iterations = world.solver_iterations as f32;

            ui.slider(hash!(), "Iterations", 1.0..256.0, &mut iterations);

            world.solver_iterations = iterations.round() as u32;
        },
    );

    // Performance overlay

    let x = screen_width() - 220.0;

    if root_ui().button(vec2(x, 20.0), "Reset Simulation") {
        reset_sim = true;
    }

    draw_text(&format!("FPS: {}", get_fps()), x, 75.0, 24.0, WHITE);

    draw_text(&format!("dt: {:.4}", dt), x, 100.0, 24.0, WHITE);

    draw_text(
        &format!("Iterations: {}", world.solver_iterations),
        x,
        125.0,
        24.0,
        WHITE,
    );

    reset_sim
}
