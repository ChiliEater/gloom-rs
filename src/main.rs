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

mod obj_parser;
mod shader;
mod util;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 400;
const INITIAL_SCREEN_H: u32 = 400;

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
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>) -> u32 {
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
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    //windowed_context.window().set_cursor_grab(glutin::window::CursorGrabMode::Confined).expect("failed to grab cursor");
    //windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

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

        // == // Set up your VAO around here
        let model_paths: Vec<String> = vec![
            "./resources/colored_panes.obj".to_string(),
            "./resources/cube.obj".to_string(),
            "./resources/square.obj".to_string(),
            "./resources/torus.obj".to_string(),
            "./resources/full_square.obj".to_string(),
            "./resources/monkey.obj".to_string(),
        ];

        let mut vaos: Vec<u32> = vec![];
        let mut models: Vec<obj_parser::Parser> = vec![];
        for path in model_paths {
            let mut parser = obj_parser::Parser::new(&path);
            let vertices = parser.flatten_vector(parser.vertices.clone());
            let indices = parser.vertex_indices();
            let colors = parser.flatten_vector(parser.colors.clone());
            let vao;
            unsafe {
                vao = create_vao(&vertices, &indices, &colors);
            }
            vaos.push(vao);
            models.push(parser);
        }

        let mut model_id: usize = 0;

        // == // Set up your shaders here

        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not, enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        let fragment_shaders: Vec<String> = vec![
            "./shaders/fragment/simple.frag".to_string(),
            "./shaders/fragment/checkerboard.frag".to_string(),
            "./shaders/fragment/circle.frag".to_string(),
            "./shaders/fragment/sine.frag".to_string(),
            "./shaders/fragment/spiral.frag".to_string(),
            "./shaders/fragment/color_change.frag".to_string(),
            "./shaders/fragment/triangle.frag".to_string(),
            ];

        let mut fragment_shader_id: usize = 0;

        let vertex_shaders: Vec<String> = vec![
            "./shaders/vertex/simple.vert".to_string(),
            "./shaders/vertex/perspective.vert".to_string(),
            "./shaders/vertex/mirror.vert".to_string(),
            "./shaders/vertex/spin.vert".to_string(),
            "./shaders/vertex/affine_transform.vert".to_string(),
            ];

        let mut vertex_shader_id: usize = 0;

        // Used to demonstrate keyboard handling for exercise 2.
        let mut model_changed = false;
        let mut fragment_shader_changed = false;
        let mut vertex_shader_changed = false;
        let mut rebuild_shaders = true;

        // Uniform variable(s) to be used in the shader
        let mut time: f32 = 0.0;
        let delta_t: f32 = 0.1; // amount to increase the time at each iteration

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            if rebuild_shaders {
                unsafe {
                    shader::ShaderBuilder::new()
                        .attach_file(fragment_shaders[fragment_shader_id].as_str())
                        .attach_file(vertex_shaders[vertex_shader_id].as_str())
                        .link()
                        .activate();
                }
                rebuild_shaders = false;
            }

            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
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

            // We tried to change the model this loop.
            let mut model_pressed = false;

            // We tried to change the fragment shader this loop.
            let mut fragment_shader_pressed = false;

            let mut vertex_shader_pressed = false;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html
                        VirtualKeyCode::A => {
                            if !model_changed {
                                if model_id == 0 {
                                    model_id = models.len();
                                }
                                model_id -= 1;
                                model_id %= models.len();

                                model_changed = true;
                                //println!("{}", model_id);
                            }
                            model_pressed = true;
                        }
                        VirtualKeyCode::D => {
                            if !model_changed {
                                model_id += 1;
                                model_id %= models.len();
                                model_changed = true;
                                //println!("{}", model_id);
                            }
                            model_pressed = true;
                        }
                        VirtualKeyCode::Q => {
                            if !fragment_shader_changed {
                                if fragment_shader_id == 0 {
                                    fragment_shader_id = fragment_shaders.len();
                                }
                                fragment_shader_id =
                                    (fragment_shader_id - 1) % fragment_shaders.len();
                                rebuild_shaders = true;
                                fragment_shader_changed = true;
                            }
                            fragment_shader_pressed = true;
                        }
                        VirtualKeyCode::E => {
                            if !fragment_shader_changed {
                                fragment_shader_id =
                                    (fragment_shader_id + 1) % fragment_shaders.len();
                                rebuild_shaders = true;
                                fragment_shader_changed = true;
                            }
                            fragment_shader_pressed = true;
                        }
                        VirtualKeyCode::Y => {
                            if !vertex_shader_changed {
                                if vertex_shader_id == 0 {
                                    vertex_shader_id = vertex_shaders.len();
                                }
                                vertex_shader_id = (vertex_shader_id - 1) % vertex_shaders.len();
                                rebuild_shaders = true;
                                vertex_shader_changed = true;
                            }
                            vertex_shader_pressed = true;
                        }
                        VirtualKeyCode::C => {
                            if !vertex_shader_changed {
                                vertex_shader_id = (vertex_shader_id + 1) % vertex_shaders.len();
                                vertex_shader_changed = true;
                                rebuild_shaders = true;
                            }
                            vertex_shader_pressed = true;
                        }

                        // default handler:
                        _ => {}
                    }
                }

                if !model_pressed {
                    model_changed = false;
                }

                if !fragment_shader_pressed {
                    fragment_shader_changed = false;
                }

                if !vertex_shader_pressed {
                    vertex_shader_changed = false;
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camera transforms here (exercise 2 & 3)

            unsafe {
                // Clear the color and depth buffers
                //gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::ClearColor(1.0, 1.0, 1.0, 1.0); // white background, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                // == // Issue the necessary gl:: commands to draw your scene here
                gl::BindVertexArray(vaos[model_id]);
                gl::DrawElements(
                    gl::TRIANGLES,
                    models[model_id].vertex_indices().len() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });

    // == //
    // == // From here on down there are only internals.
    // == //

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!(
                    "New window size received: {}x{}",
                    physical_size.width, physical_size.height
                );
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Q => {
                        //*control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}
