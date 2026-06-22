use macroquad::prelude::*;

pub struct CameraController {
    pub position: Vec2,
    pub zoom: f32,
    pub last_mouse_pos: Vec2,
}
impl CameraController {
    pub fn build_camera(&self) -> Camera2D {
        Camera2D {
            target: self.position,
            zoom: vec2(self.zoom / screen_width(), -self.zoom / screen_height()),
            ..Default::default()
        }
    }
}
pub fn move_camera(camera: &mut CameraController, dt: f32) {
    let speed = 500.0;

    if is_key_down(KeyCode::W) {
        camera.position.y += speed * dt;
    }

    if is_key_down(KeyCode::S) {
        camera.position.y -= speed * dt;
    }

    if is_key_down(KeyCode::A) {
        camera.position.x -= speed * dt;
    }

    if is_key_down(KeyCode::D) {
        camera.position.x += speed * dt;
    }
}

pub fn zoom_camera(camera: &mut CameraController) {
    let (_, wheel_y) = mouse_wheel();

    camera.zoom *= 1.0 + wheel_y * 0.1;
    camera.zoom = camera.zoom.clamp(0.005, 50.0);
}

pub fn drag_camera(camera: &mut CameraController, cam2d: &Camera2D) {
    let mouse = vec2(mouse_position().0, mouse_position().1);

    if is_mouse_button_pressed(MouseButton::Middle) {
        camera.last_mouse_pos = mouse;
    }

    if is_mouse_button_down(MouseButton::Middle) {
        let world_now = cam2d.screen_to_world(mouse);
        let world_prev = cam2d.screen_to_world(camera.last_mouse_pos);
        let world_delta = world_now - world_prev;

        camera.position -= world_delta;

        camera.last_mouse_pos = mouse;
    }
}

pub fn mouse_world(camera: &Camera2D) -> Vec2 {
    camera.screen_to_world(mouse_position().into())
}
