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
use input::input_loop::{self, InputLoop};
use render::meshes::Meshes;
use render::rendering_loop::RenderingLoop;
use render::window_locks::{self, WindowLocks};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::{mem, os::raw::c_void, ptr};

mod input;
mod render;
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

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));
    let context_builder = glutin::ContextBuilder::new().with_vsync(true);
    let window_context = context_builder
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let window_locks = WindowLocks::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H);
    let arc_window_locks = Arc::new(window_locks);
    let window_locks_render = Arc::clone(&arc_window_locks);
    let window_locks_input = Arc::clone(&arc_window_locks);


    let model_paths: Vec<String> = vec![
        //"./resources/lunarsurface.obj".to_string(),
        "./resources/helicopter.obj".to_string(),
        //"./resources/cube.obj".to_string(),
    ];
    let mut models = Meshes::new();
    models.add_all(&model_paths);

    // Spawn render thread
    let render_thread = thread::spawn(move || {
        let mut rendering_loop = RenderingLoop::new(&window_locks_render, window_context, models);
        rendering_loop.enable_mouse_input();
        rendering_loop.start();
    });

    // Watch render thread health
    let health = watch_health(render_thread);

    // Spawn input thread
    let input_loop = InputLoop::new(health, &window_locks_input);
    input_loop.start(&mut event_loop);
}

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

pub fn watch_health(render_thread: JoinHandle<()>) -> Arc<RwLock<bool>> {
    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if render_thread.join().is_err() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });
    render_thread_healthy
}
