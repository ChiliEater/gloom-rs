extern crate nalgebra_glm as glm;
use std::{f32::consts, f64::consts::PI};

use glm::{pi, two_pi, vec2, vec3, vec4, Mat4x4, Vec3, Vec4, mat2, Mat2x2, Vec2};

use crate::input::controls::{MAX_ANGLE, MAX_SPEED};

pub const BASE_ROTATION: f32 = consts::PI * 2.0;
pub const ROTATION_RATE: f32 = BASE_ROTATION * 2.0;

pub struct Heading {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,  // measured in radians
    pub pitch: f32, // measured in radians
    pub yaw: f32,   // measured in radians
    pub top_rotor: f32,
    pub rear_rotor: f32,
}

pub fn simple_heading_animation(time: f32) -> Heading {
    let t = time as f64;
    let step = 0.05f64;
    let path_size = 50f64;
    let circuit_speed = 0.5f64;

    let xpos = path_size * (2.0 * (t + 0.0) * circuit_speed).sin();
    let xpos_next = path_size * (2.0 * (t + step) * circuit_speed).sin();
    let zpos = 3.0 * path_size * ((t + 0.0) * circuit_speed).cos();
    let zpos_next = 3.0 * path_size * ((t + step) * circuit_speed).cos();

    let delta_pos = glm::vec2(xpos_next - xpos, zpos_next - zpos);

    let roll = (t * circuit_speed).cos() * 0.5;
    let pitch = -0.175 * glm::length(&delta_pos);
    let yaw = PI + delta_pos.x.atan2(delta_pos.y);

    Heading {
        x: xpos as f32,
        y: 10 as f32,
        z: zpos as f32,
        roll: roll as f32,
        pitch: pitch as f32,
        yaw: yaw as f32,
        top_rotor: 0.0,
        rear_rotor: 0.0,
    }
}

pub fn movement_animation(
    speed: &Vec3,
    heli_position: &Vec3,
    heli_rotation: &Vec3,
    camera_rotation: &Vec3,
) -> Heading {
    let new_position: Vec3 = heli_position + speed;

    let xpos: f32 = new_position.x;
    let ypos: f32 = new_position.y;
    let zpos: f32 = new_position.z;
    
    let theta: f32 = heli_rotation.y;

    let relative_rotation: Mat2x2 = mat2(
        theta.cos(),-theta.sin(),
        theta.sin(),theta.cos()
    );

    let relative_speed: Vec2 = (relative_rotation * vec2(speed.x,speed.z)).xy();
    let roll: f32 = -clamp(relative_speed.x / MAX_SPEED * MAX_ANGLE, -MAX_ANGLE, MAX_ANGLE);
    let pitch: f32 = clamp(relative_speed.y / MAX_SPEED * MAX_ANGLE, -MAX_ANGLE, MAX_ANGLE);
    println!("[Speed  |  Roll/Pitch] \n{:.3}",mat2(relative_speed.x,roll,relative_speed.y,pitch));
    
    
    
    // THIS WORKS
    let error: f32 = get_angular_error(heli_rotation.y, camera_rotation.y);
    let yaw: f32 = (heli_rotation.y - 0.05 * error) % two_pi::<f32>();

    let top_rotor: f32 = BASE_ROTATION + glm::magnitude(speed) * ROTATION_RATE * 0.5;
    let rear_rotor: f32 = BASE_ROTATION + (yaw - heli_rotation.y) * ROTATION_RATE * 4.0;

    Heading {
        x: xpos,
        y: ypos,
        z: zpos,
        roll,
        pitch,
        yaw,
        top_rotor,
        rear_rotor,
    }
}

pub fn rotate_all(angles: &Vec3) -> Mat4x4 {
    let x_axis: Vec3 = vec3(1.0, 0.0, 0.0);
    let y_axis: Vec3 = vec3(0.0, 1.0, 0.0);
    let z_axis: Vec3 = vec3(0.0, 0.0, 1.0);

    let z_rotation: Mat4x4 = glm::rotation(angles.z, &z_axis);
    let y_rotation: Mat4x4 = glm::rotation(angles.y, &y_axis);
    let x_rotation: Mat4x4 = glm::rotation(angles.x, &x_axis);
    

    z_rotation * y_rotation * x_rotation
}

pub fn rotate_all_intrinsic(angles: &Vec3) -> Mat4x4 {
    let x_axis: Vec3 = vec3(1.0, 0.0, 0.0);
    let y_axis: Vec3 = vec3(0.0, 1.0, 0.0);
    let z_axis: Vec3 = vec3(0.0, 0.0, 1.0);

    let z_rotation: Mat4x4 = glm::rotation(angles.z, &z_axis);
    let y_rotation: Mat4x4 = glm::rotation(angles.y, &y_axis);
    let x_rotation: Mat4x4 = glm::rotation(angles.x, &x_axis);
    

    x_rotation * y_rotation * z_rotation
}

pub fn rotate_around(angles: &Vec3, point: &Vec3) -> Mat4x4 {
    glm::translation(&point) * rotate_all(angles) * glm::translation(&(-point))
}

pub fn rotate_around_intrinsic(angles: &Vec3, point: &Vec3) -> Mat4x4 {
    glm::translation(&point) * rotate_all_intrinsic(angles) * glm::translation(&(-point))
}

pub fn scale_around(factors: &Vec3, point: &Vec3) -> Mat4x4 {
    glm::translation(point) * glm::scaling(factors) * glm::translation(&(-point))
}

pub fn to_homogeneous(vec: &Vec3) -> Vec4 {
    vec4(vec.x, vec.y, vec.z, 1.0)
}

fn clamp(x: f32, min_value: f32, max_value: f32) -> f32 {
    x.min(max_value).max(min_value)
}

fn get_angular_error(angle1: f32, angle2: f32) -> f32 {
    let diff: f32 = angle1 - (2.0 * pi::<f32>() - angle2);

    let mut error = if diff > pi::<f32>() {
        diff - two_pi::<f32>()
    } else if diff < -pi::<f32>() {
        diff + two_pi::<f32>()
    } else {
        diff
    };

    // Ensure the result is within the -pi to pi range
    if error > pi::<f32>() {
        error -= two_pi::<f32>();
    } else if error < -pi::<f32>() {
        error += two_pi::<f32>();
    }

    error
}
