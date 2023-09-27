use std::sync::{Arc, Mutex};

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use nalgebra_glm::{vec3, Mat3x3, Mat4x4, Vec3, Vec4, pi};

use crate::render::window_locks::WindowLocks;

const X_SENSITIVITY: f32 = 60.0;
const Y_SENSITIVITY: f32 = 60.0;
const MOVEMENT_SPEED: f32 = 50.0;
const SPRINT_MULTIPLIER: f32 = 10.0;

pub struct Controls {
    position: Vec3,
    rotation: Vec3,
    x_axis: Vec3,
    y_axis: Vec3,
    z_axis: Vec3,
    zero: Vec3,
    window_size: Arc<Mutex<(u32, u32, bool)>>,
    pressed_keys: Arc<Mutex<Vec<VirtualKeyCode>>>,
    mouse_delta: Arc<Mutex<(f32, f32)>>,
}

impl Controls {
    pub fn new(window_locks: &WindowLocks) -> Controls {
        Controls {
            position: vec3(0.0, 0.0, 2.0),
            rotation: vec3(0.0, 0.0, 0.0),
            x_axis: vec3(1.0, 0.0, 0.0),
            y_axis: vec3(0.0, 1.0, 0.0),
            z_axis: vec3(0.0, 0.0, 1.0),
            zero: vec3(0.0, 0.0, 0.0),
            window_size: window_locks.window_size(),
            pressed_keys: window_locks.pressed_keys(),
            mouse_delta: window_locks.mouse_delta(),
        }
    }

    pub fn handle(&mut self, delta_time: f32) -> Mat4x4 {
        let inverse_rotation = self.handle_mouse(delta_time);
        let translation = self.handle_keyboard(delta_time, &inverse_rotation);
        let rotation: Mat4x4 = glm::rotation(self.rotation.x, &self.x_axis)* glm::rotation(self.rotation.y, &self.y_axis);
        rotation * translation
    }

    fn handle_mouse(&mut self, delta_time: f32) -> Mat4x4 {
        // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
        if let Ok(mut delta) = self.mouse_delta.lock() {
            // == // Optionally access the accumulated mouse movement between
            // == // frames here with `delta.0` and `delta.1`
            if let Ok(screen) = self.window_size.lock() {
                self.rotation += vec3(
                    delta.1 / screen.1 as f32 * pi::<f32>() * delta_time * X_SENSITIVITY,
                    delta.0 / screen.0 as f32 * pi::<f32>() * delta_time * Y_SENSITIVITY,
                    0.0,
                );
            }
            *delta = (0.0, 0.0); // reset when done
        }

        self.rotation.x = (glm::max(
            &glm::min(&self.rotation, glm::half_pi()),
            -glm::half_pi::<f32>(),
        ))
        .x;

        self.rotation.y %= glm::two_pi::<f32>();

        glm::rotation(self.rotation.x * -1.0, &self.x_axis)
            * glm::rotation(self.rotation.y * -1.0, &self.y_axis)
    }

    fn handle_keyboard(&mut self, delta_time: f32, inverse_rotation_matrix: &Mat4x4) -> Mat4x4 {
        if let Ok(keys) = self.pressed_keys.lock() {
            let mut delta_speed = MOVEMENT_SPEED * delta_time;
            if keys.contains(&LShift) {
                delta_speed *= SPRINT_MULTIPLIER;
            }
            const X_SENSITIVITY: f32 = 7.0;
            const Y_SENSITIVITY: f32 = 7.0;
            for key in keys.iter() {
                match key {
                    D | L => {
                        self.position += (inverse_rotation_matrix
                            * (self.x_axis.to_homogeneous() * delta_speed))
                            .xyz()
                    }
                    A | J => {
                        self.position -= (inverse_rotation_matrix
                            * (self.x_axis.to_homogeneous() * delta_speed))
                            .xyz()
                    }
                    Space => self.position += self.y_axis * delta_speed,
                    LControl => self.position -= self.y_axis * delta_speed,
                    S | K => {
                        self.position += (inverse_rotation_matrix
                            * (self.z_axis.to_homogeneous() * delta_speed))
                            .xyz()
                    }
                    W | I => {
                        self.position -= (inverse_rotation_matrix
                            * (self.z_axis.to_homogeneous() * delta_speed))
                            .xyz()
                    }
                    Left => self.rotation.y -= Y_SENSITIVITY * delta_time,
                    Right => self.rotation.y += Y_SENSITIVITY * delta_time,
                    Up => self.rotation.x -= X_SENSITIVITY * delta_time,
                    Down => self.rotation.x += X_SENSITIVITY * delta_time,
                    _ => {}
                }
            }
        }
        glm::translation(&(self.position * -1.0))
    }
}
