// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr};
use input::input_loop::{self, InputLoop};
use render::rendering_loop::RenderingLoop;
use render::window_locks::{self, WindowLocks};

mod input;
mod render;
mod obj_parser;
mod shader;
mod util;

use glm::{pi, vec3, Mat4x4};
use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 800;
const MOVEMENT_SPEED: f32 = 2.0;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  pointer_to_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

// == // Generate your VAO here
pub unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>) -> u32 {
    // Generate array & store ID
    let mut vao_id: u32 = 0;
    gl::GenVertexArrays(1, &mut vao_id);

    // Bind VAO
    gl::BindVertexArray(vao_id);

    // Generate buffer & store ID
    let mut vbo_id: u32 = 0;
    gl::GenBuffers(1, &mut vbo_id);

    // Bind VBO
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);

    // Fill VBO
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );

    // Setup VAP (clean this up?)
    let attribute_index = 0;
    gl::VertexAttribPointer(attribute_index, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());

    // Enable VBO
    gl::EnableVertexAttribArray(attribute_index);

    // Generate index buffer & ID
    let mut ibo_id: u32 = 0;
    gl::GenBuffers(1, &mut ibo_id);

    // Bind IBO
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_id);

    // Fill IBO
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    let mut color_id: u32 = 0;
    gl::GenBuffers(1, &mut color_id);

    gl::BindBuffer(gl::ARRAY_BUFFER, color_id);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(colors),
        pointer_to_array(colors),
        gl::STATIC_DRAW,
    );

    let color_attribute = 2;
    gl::VertexAttribPointer(color_attribute, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(color_attribute);

    vao_id
}

fn main() {
    let x_axis: glm::Vec3 = glm::vec3(1.0, 0.0, 0.0);
    let y_axis: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);
    let z_axis: glm::Vec3 = glm::vec3(0.0, 0.0, 1.0);
    let origin: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
    // Set up the necessary objects to deal with windows and event handling
    let window_locks = WindowLocks::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H);

    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let window_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let rendering_loop = RenderingLoop::new(window_context, &window_locks);
    rendering_loop.start();
    // == //
    // == // From here on down there are only internals.
    // == //
    let health = rendering_loop.watch_health();

    let input_loop = InputLoop::new(el, health, &window_locks);
    input_loop.start();
}
