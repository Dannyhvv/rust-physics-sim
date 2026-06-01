use crate::sandbox::tools::{ActiveTool, DragTool, RectTool};
use crate::vec2;
use macroquad::ui::*;

pub fn draw_ui(active_tool: &mut ActiveTool, drag_tool: &mut DragTool, rect_tool: &mut RectTool) {
    root_ui().window(hash!(), vec2(10., 10.), vec2(250., 200.), |ui| {
        if ui.button(None, "Drag Tool") {
            *active_tool = ActiveTool::Drag;
        }

        if ui.button(None, "Rectangle Tool") {
            *active_tool = ActiveTool::Rectangle;
        }

        match active_tool {
            ActiveTool::Drag => {
                ui.separator();

                ui.label(None, "Drag Tool");

                ui.slider(hash!(), "Strength", 0.0..100.0, &mut drag_tool.strength);

                ui.slider(hash!(), "Damping", 0.0..50.0, &mut drag_tool.damping);
            }

            ActiveTool::Rectangle => {
                ui.separator();

                ui.label(None, "Rectangle Tool");

                ui.slider(hash!(), "Density", 1.0..100.0, &mut rect_tool.density);

                ui.slider(hash!(), "Restitution", 0.0..1.0, &mut rect_tool.restitution);

                ui.checkbox(hash!(), "Static", &mut rect_tool.is_static);

                ui.checkbox(hash!(), "Collide", &mut rect_tool.can_collide);

                ui.checkbox(hash!(), "Random Colors", &mut rect_tool.random_colors);
            }
        }
    });
}
