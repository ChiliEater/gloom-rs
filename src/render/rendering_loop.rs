extern crate nalgebra_glm as glm;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::{mem, os::raw::c_void, ptr};

use crate::input::controls::Controls;
use crate::{obj_parser, shader, util, INITIAL_SCREEN_H, INITIAL_SCREEN_W};
use glm::{pi, vec3, Mat4x4};
use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;
use glutin::window::Window;
use glutin::{ContextWrapper, NotCurrent, PossiblyCurrent};
use tobj::Model;

use super::meshes::Meshes;
use super::window_locks::WindowLocks;

pub struct RenderingLoop {
    window_size: Arc<Mutex<(u32, u32, bool)>>,
    context: ContextWrapper<PossiblyCurrent, Window>,
    window_aspect_ratio: f32,
    models: Meshes,
    controls: Controls,
}

impl RenderingLoop {
    pub fn new(
        window_locks: &WindowLocks,
        window_context: ContextWrapper<NotCurrent, Window>,
        models: Meshes,
    ) -> RenderingLoop {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = window_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        RenderingLoop {
            window_size: window_locks.window_size(),
            context,
            window_aspect_ratio: INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32,
            models,
            controls: Controls::new(window_locks),
        }
    }

    pub fn start(&mut self) {
        self.configure_opengl();
        self.models.generate_vaos();

        // == // Set up your shaders here
        let fragment_shaders: Vec<String> = vec![
            "./shaders/fragment/simple.frag".to_string(),
            "./shaders/fragment/checkerboard.frag".to_string(),
            "./shaders/fragment/circle.frag".to_string(),
            "./shaders/fragment/sine.frag".to_string(),
            "./shaders/fragment/spiral.frag".to_string(),
            "./shaders/fragment/color_change.frag".to_string(),
            "./shaders/fragment/triangle.frag".to_string(),
        ];

        let vertex_shaders: Vec<String> = vec![
            "./shaders/vertex/simple.vert".to_string(),
            "./shaders/vertex/perspective.vert".to_string(),
            "./shaders/vertex/mirror.vert".to_string(),
            "./shaders/vertex/spin.vert".to_string(),
            "./shaders/vertex/affine_transform.vert".to_string(),
        ];

        // Uniform variable(s) to be used in the shader
        let mut time: f32 = 0.0;
        let delta_t: f32 = 0.1; // amount to increase the time at each iteration

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            unsafe {
                shader::ShaderBuilder::new()
                    .attach_file(fragment_shaders[0].as_str())
                    .attach_file(vertex_shaders[0].as_str())
                    .link()
                    .activate();
            }

            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = self.window_size.lock() {
                if new_size.2 {
                    self.context
                        .resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    self.window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    new_size.2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe {
                        gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32);
                    }
                }
            }

            // Update the uniform variables
            unsafe {
                time += delta_t; // Update the time value
                gl::Uniform1f(1, time);
            }

            let perspective_matrix: Mat4x4 =
                glm::perspective(self.window_aspect_ratio, glm::half_pi(), 0.25, 100.0);

            unsafe {
                let transform_matrix: Mat4x4 =
                    perspective_matrix * self.controls.handle(delta_time);

                gl::UniformMatrix4fv(3, 1, gl::FALSE, transform_matrix.as_ptr());
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                                                          //gl::ClearColor(1.0, 1.0, 1.0, 1.0); // white background, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                // == // Issue the necessary gl:: commands to draw your scene here
                self.models.draw();
            }

            // Display the new color buffer on the display
            self.context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    }

    pub fn enable_mouse_input(&self) {
        self.context
            .window()
            .set_cursor_grab(glutin::window::CursorGrabMode::Confined)
            .expect("failed to grab cursor");
        self.context.window().set_cursor_visible(false);
    }

    pub fn disable_mouse_input(&self) {
        self.context
            .window()
            .set_cursor_grab(glutin::window::CursorGrabMode::None)
            .expect("failed to grab cursor");
        self.context.window().set_cursor_visible(true);
    }

    fn configure_opengl(&self) {
        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!(
                "{}: {}",
                util::get_gl_string(gl::VENDOR),
                util::get_gl_string(gl::RENDERER)
            );
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!(
                "GLSL\t: {}",
                util::get_gl_string(gl::SHADING_LANGUAGE_VERSION)
            );
        }
    }
}
