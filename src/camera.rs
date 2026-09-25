use glam::{Mat4, Vec3};
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window};

const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
]);

const PITCH_LIMIT_DEG: f32 = 89.0;
const MOVE_SPEED: f32 = 12.0;
const MOUSE_SENSITIVITY: f32 = 0.0022;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [f32; 16],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.view_proj().to_cols_array();
    }
}

pub struct Camera {
    pub position: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub aspect: f32,
    pub fov_y_deg: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        let position = Vec3::new(24.0, 18.0, 32.0);
        let toward_origin = (-position).normalize();
        Self {
            position,
            pitch: toward_origin.y.asin(),
            yaw: toward_origin.x.atan2(-toward_origin.z),
            aspect,
            fov_y_deg: 60.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect.max(f32::EPSILON);
    }

    pub fn forward(&self) -> Vec3 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        Vec3::new(sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch).normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    pub fn view_proj(&self) -> Mat4 {
        let view = Mat4::look_to_rh(self.position, self.forward(), Vec3::Y);
        let proj = Mat4::perspective_rh(
            self.fov_y_deg.to_radians(),
            self.aspect,
            self.znear,
            self.zfar,
        );
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[derive(Default)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    pub pointer_locked: bool,
}

impl CameraController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_keyboard(&mut self, event: &KeyEvent, window: &Window) -> bool {
        let PhysicalKey::Code(code) = event.physical_key else {
            return false;
        };

        let pressed = event.state == ElementState::Pressed;
        let amount = if pressed { 1.0 } else { 0.0 };

        match code {
            KeyCode::KeyW => {
                self.amount_forward = amount;
                true
            }
            KeyCode::KeyS => {
                self.amount_backward = amount;
                true
            }
            KeyCode::KeyA => {
                self.amount_left = amount;
                true
            }
            KeyCode::KeyD => {
                self.amount_right = amount;
                true
            }
            KeyCode::Space => {
                self.amount_up = amount;
                true
            }
            KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                self.amount_down = amount;
                true
            }
            KeyCode::Escape if pressed => {
                self.unlock_pointer(window);
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse_button(&mut self, button: MouseButton, state: ElementState, window: &Window) -> bool {
        if button == MouseButton::Left && state == ElementState::Pressed && !self.pointer_locked {
            self.lock_pointer(window);
            return true;
        }
        false
    }

    pub fn process_mouse_delta(&self, camera: &mut Camera, delta: (f64, f64)) {
        if !self.pointer_locked {
            return;
        }

        let (dx, dy) = delta;
        camera.yaw += dx as f32 * MOUSE_SENSITIVITY;
        camera.pitch -= dy as f32 * MOUSE_SENSITIVITY;

        let limit = PITCH_LIMIT_DEG.to_radians();
        camera.pitch = camera.pitch.clamp(-limit, limit);
    }

    pub fn update_camera(&self, camera: &mut Camera, dt: f32) {
        let forward = camera.forward();
        let right = camera.right();
        let mut wish = Vec3::ZERO;
        wish += forward * (self.amount_forward - self.amount_backward);
        wish += right * (self.amount_right - self.amount_left);
        wish += Vec3::Y * (self.amount_up - self.amount_down);

        if wish.length_squared() > 0.0 {
            camera.position += wish.normalize() * MOVE_SPEED * dt;
        }
    }

    pub fn lock_pointer(&mut self, window: &Window) {
        let grab = window
            .set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined));

        if grab.is_ok() {
            window.set_cursor_visible(false);
            self.pointer_locked = true;
        } else {
            log::warn!("Failed to grab cursor; mouse look may be unavailable");
        }
    }

    pub fn unlock_pointer(&mut self, window: &Window) {
        let _ = window.set_cursor_grab(CursorGrabMode::None);
        window.set_cursor_visible(true);
        self.pointer_locked = false;
        self.amount_left = 0.0;
        self.amount_right = 0.0;
        self.amount_forward = 0.0;
        self.amount_backward = 0.0;
        self.amount_up = 0.0;
        self.amount_down = 0.0;
    }
}
