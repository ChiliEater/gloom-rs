use std::sync::{Arc, Mutex};

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use nalgebra_glm::{pi, vec3, Mat3x3, Mat4x4, Vec3, Vec4, two_pi};

use crate::render::window_locks::WindowLocks;
use crate::toolbox::{rotate_all, rotate_around, scale_around, to_homogeneous};

const X_SENSITIVITY: f32 = 60.0;
const Y_SENSITIVITY: f32 = 60.0;
const MOVEMENT_SPEED: f32 = 50.0;
const SPRINT_MULTIPLIER: f32 = 2.0;
pub const MAX_ANGLE: f64 = 0.5; // Max angle in radians
pub const MAX_SPEED: f64 = 25.0; // Max speed from the inputs

const KEY_W: u32 = 17;
const KEY_A: u32 = 30;
const KEY_S: u32 = 31;
const KEY_D: u32 = 32;

pub struct Controls {
    position: Vec3,
    pub rotation: Vec3,
    x_axis: Vec3,
    y_axis: Vec3,
    z_axis: Vec3,
    window_size: Arc<Mutex<(u32, u32, bool)>>,
    pressed_keys: Arc<Mutex<Vec<KeyboardInput>>>,
    mouse_delta: Arc<Mutex<(f32, f32)>>,
    pub speed: Vec3,
}

impl Controls {
    pub fn new(window_locks: &WindowLocks) -> Controls {
        Controls {
            position: vec3(0.0, 0.0, 2.0),
            rotation: Vec3::zeros(),
            x_axis: vec3(1.0, 0.0, 0.0),
            y_axis: vec3(0.0, 1.0, 0.0),
            z_axis: vec3(0.0, 0.0, 1.0),
            speed: Vec3::zeros(),
            window_size: window_locks.window_size(),
            pressed_keys: window_locks.pressed_keys(),
            mouse_delta: window_locks.mouse_delta(),
        }
    }

    pub fn handle(&mut self, delta_time: f32, pivot: &Vec3) -> Mat4x4 {
        //let translation = self.handle_keyboard(delta_time, &negative_rotation);
        let translation = glm::translation(&(self.position * -1.0));
        let rotation: Mat4x4 = rotate_around(&self.rotation, pivot);
        rotation * translation
    }

    pub fn handle_mouse(&mut self, delta_time: f32, pivot: &Vec3) -> Mat4x4 {
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
            &glm::min(&self.rotation, glm::half_pi::<f32>() - 0.1),
            -glm::half_pi::<f32>() + 0.1,
        ))
        .x;
        self.rotation.y = ((self.rotation.y + two_pi::<f32>())%two_pi::<f32>()).abs().min(two_pi());
        

        rotate_around(&self.rotation, &pivot)
    }

    fn handle_keyboard(&mut self, delta_time: f32, negative_rotation_matrix: &Mat4x4) -> Mat4x4 {
        let mut delta_speed = MOVEMENT_SPEED * delta_time;
        if let Ok(inputs) = self.pressed_keys.lock() {
            let virtual_keys: Vec<VirtualKeyCode> = inputs
                .iter()
                .map(|input| input.virtual_keycode.unwrap())
                .collect();
            let scancodes: Vec<u32> = inputs.iter().map(|input| input.scancode).collect();
            if virtual_keys.contains(&LShift) {
                delta_speed *= SPRINT_MULTIPLIER;
            }

            const X_SENSITIVITY: f32 = 7.0;
            const Y_SENSITIVITY: f32 = 7.0;
            for input in virtual_keys.iter() {
                match input {
                    Space => self.position += self.y_axis * delta_speed,
                    LControl => self.position -= self.y_axis * delta_speed,
                    Left => self.rotation.y -= Y_SENSITIVITY * delta_time,
                    Right => self.rotation.y += Y_SENSITIVITY * delta_time,
                    Up => self.rotation.x -= X_SENSITIVITY * delta_time,
                    Down => self.rotation.x += X_SENSITIVITY * delta_time,
                    _ => {}
                }
            }

            for code in scancodes.iter() {
                match *code {
                    KEY_D => {
                        self.position += (negative_rotation_matrix
                            * (to_homogeneous(&self.x_axis) * delta_speed))
                            .xyz()
                    }
                    KEY_A => {
                        self.position -= (negative_rotation_matrix
                            * (to_homogeneous(&self.x_axis) * delta_speed))
                            .xyz()
                    }
                    KEY_S => {
                        self.position += (negative_rotation_matrix
                            * (to_homogeneous(&self.z_axis) * delta_speed))
                            .xyz()
                    }
                    KEY_W => {
                        self.position -= (negative_rotation_matrix
                            * (to_homogeneous(&self.z_axis) * delta_speed))
                            .xyz()
                    }
                    _ => {}
                }
            }
        }

        glm::translation(&(self.position * -1.0))
    }

    pub fn handle_keyboard_helicopter(&mut self, delta_time: f32, pivot: &Vec3) {
        let camera_transform = rotate_around(&self.rotation, pivot);
        let deceleration: f32 = 1.0;
        let acceleration: f32 = deceleration * 10.0;
        //self.speed = glm::max(&self.speed, max_speed);
        let mut delta_acceleration = acceleration * delta_time;
        let mut delta_deceleration = deceleration * delta_time;
        if let Ok(inputs) = self.pressed_keys.lock() {
            let virtual_keys: Vec<VirtualKeyCode> = inputs
                .iter()
                .map(|input| input.virtual_keycode.unwrap())
                .collect();
            let scancodes: Vec<u32> = inputs.iter().map(|input| input.scancode).collect();
            if virtual_keys.contains(&LShift) {
                delta_acceleration *= SPRINT_MULTIPLIER;
            }

            const X_SENSITIVITY: f32 = 7.0;
            const Y_SENSITIVITY: f32 = 7.0;
            for input in virtual_keys.iter() {
                match input {
                    Space => self.speed.y += delta_acceleration,
                    LControl => self.speed.y -= delta_acceleration,
                    Left => self.rotation.y -= Y_SENSITIVITY * delta_time,
                    Right => self.rotation.y += Y_SENSITIVITY * delta_time,
                    Up => self.rotation.x -= X_SENSITIVITY * delta_time,
                    Down => self.rotation.x += X_SENSITIVITY * delta_time,
                    _ => {}
                }
            }

            for code in scancodes.iter() {
                match *code {
                    KEY_D => {
                        self.speed += (camera_transform
                            * (to_homogeneous(&self.x_axis) * delta_acceleration))
                            .xyz()
                    }
                    KEY_A => {
                        self.speed -= (camera_transform
                            * (to_homogeneous(&self.x_axis) * delta_acceleration))
                            .xyz()
                    }
                    KEY_S => {
                        self.speed += (camera_transform
                            * (to_homogeneous(&self.z_axis) * delta_acceleration))
                            .xyz()
                    }
                    KEY_W => {
                        self.speed -= (camera_transform
                            * (to_homogeneous(&self.z_axis) * delta_acceleration))
                            .xyz()
                    }
                    _ => {}
                }
            }
        }
        //self.speed += (-glm::normalize(&self.speed)) * delta_deceleration;
        self.speed = glm::clamp(&self.speed, -MAX_SPEED as f32, MAX_SPEED as f32);
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
}
