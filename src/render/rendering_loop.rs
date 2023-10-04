extern crate nalgebra_glm as glm;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::{mem, os::raw::c_void, ptr};

use crate::input::controls::Controls;
use crate::shader::Shader;
use crate::toolbox::{
    rotate_all, rotate_around, scale_around, simple_heading_animation, to_homogeneous,
};
use crate::{shader, util, INITIAL_SCREEN_H, INITIAL_SCREEN_W};
use glm::{cos, half_pi, pi, sin, vec3, Mat4x4, Vec3};
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

use super::mesh::{Helicopter, Terrain};
use super::meshes::Meshes;
use super::scene_graph::{self, Node, NodeType, SceneNode};
use super::vao::create_vao;
use super::window_locks::WindowLocks;

const TERRAIN: &str = "./resources/lunarsurface.obj";
const HELICOPTER: &str = "./resources/helicopter.obj";
const COLORCUBE: &str = "./resources/cube.obj";
const HELI_COUNT: u32 = 2;
const HELICOPTER_INDEX: usize = 2;

pub struct RenderingLoop {
    window_size: Arc<Mutex<(u32, u32, bool)>>,
    context: ContextWrapper<PossiblyCurrent, Window>,
    window_aspect_ratio: f32,
    meshes: Meshes,
    controls: Controls,
    shader: Option<Shader>,
}

impl RenderingLoop {
    pub fn new(
        window_locks: &WindowLocks,
        window_context: ContextWrapper<NotCurrent, Window>,
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
            meshes: Meshes::new(),
            controls: Controls::new(window_locks),
            shader: None,
        }
    }
    pub fn start(&mut self) {
        self.configure_opengl();
        self.meshes.generate_vaos();
        let mut binding = self.setup_scene();
        let mut root_node = binding.as_mut();

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
            "./shaders/vertex/sphere.vert".to_string(),
            "./shaders/vertex/simple.vert".to_string(),
            "./shaders/vertex/perspective.vert".to_string(),
            "./shaders/vertex/mirror.vert".to_string(),
            "./shaders/vertex/spin.vert".to_string(),
            "./shaders/vertex/affine_transform.vert".to_string(),
        ];

        // Uniform variable(s) to be used in the shader
        let mut time: f32 = 0.0;
        let delta_t: f32 = 0.1; // amount to increase the time at each iteration

        self.shader = Some(unsafe {
            shader::ShaderBuilder::new()
                .attach_file(fragment_shaders[0].as_str())
                .attach_file(vertex_shaders[0].as_str())
                .link()
        });
        unsafe { self.shader.as_mut().unwrap().activate() };

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            helicopter_animation(elapsed, &mut root_node);

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
            let perspective_matrix: Mat4x4 =
                glm::perspective(self.window_aspect_ratio, glm::half_pi(), 0.25, 3000.0);

            unsafe {
                // Clear the color and depth buffers
                match &self.shader {
                    Some(shader) => {
                        let mut uniform;
                        uniform = shader.get_uniform_location("camera_position");
                        let camera_position = self.controls.get_position();
                        gl::Uniform4f(
                            uniform,
                            camera_position.x,
                            camera_position.y,
                            camera_position.z,
                            1.0,
                        );
                        uniform = shader.get_uniform_location("elapsed_time");
                        gl::Uniform1f(uniform, elapsed);
                    }
                    None => {}
                }
                //gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::ClearColor(0.9216, 0.4431, 0.1451, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                // == // Issue the necessary gl:: commands to draw your scene here
                let camera_offset = vec3(0.0, 10.0, 20.0);
                self.handle_camera(&mut root_node, &camera_offset);
                let movement = self.controls.handle(delta_time, &-camera_offset);
                self.draw_scene(
                    &root_node,
                    &(perspective_matrix * movement),
                    &glm::identity(),
                );
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

    fn setup_scene(&self) -> Node {
        let helicopter_mesh = Helicopter::load(HELICOPTER);
        let terrain_mesh = Terrain::load(TERRAIN);

        let mut root_node: Node = SceneNode::new(NodeType::Mesh);

        // Add terrain as a child of root.
        root_node.add_child(&SceneNode::from_vao(
            unsafe { create_vao(&terrain_mesh) },
            terrain_mesh.index_count,
        ));

        let heli_vao = unsafe { create_vao(&helicopter_mesh.body) };
        let heli_parts_vaos: Vec<u32> = vec![
            unsafe { create_vao(&helicopter_mesh.main_rotor) },
            unsafe { create_vao(&helicopter_mesh.tail_rotor) },
            unsafe { create_vao(&helicopter_mesh.door) },
        ];
        for j in 0..HELI_COUNT {
            // Add helicopter body
            let mut helicopter_body_node =
                SceneNode::from_vao(heli_vao, helicopter_mesh.body.index_count);
            // Try some transformations on helicopter
            helicopter_body_node.scale = vec3(1.0, 1.0, 1.0) * 1.0;
            helicopter_body_node.position = vec3(0.0, 20.0, -400.0);
            helicopter_body_node.reference_point = vec3(0.0, 20.0, -20.0);

            // Add helicopter as a child of root.
            root_node.add_child(&helicopter_body_node);

            // Add the rest of helicopter parts as children of body
            for i in 0..3 {
                helicopter_body_node.add_child(&SceneNode::from_vao(
                    heli_parts_vaos[i],
                    helicopter_mesh[i + 1].index_count,
                ));
            }

            helicopter_body_node.get_child(1).reference_point = vec3(0.35, 2.3, 10.40) * 1.0;
            helicopter_body_node.get_child(1).rotation = vec3(0.0, 0.0, 0.0);
        }

        // Add camera to heli
        let mut camera: Node = SceneNode::new(NodeType::Camera);
        camera.position = vec3(0.0, 10.0, -20.0);
        camera.rotation = vec3(0.05, 0.0, 0.0);
        root_node.get_child(1).add_child(&camera);

        root_node
    }

    unsafe fn draw_scene(
        &mut self,
        node: &SceneNode,
        view_projection_matrix: &Mat4x4,
        transformation_so_far: &Mat4x4,
    ) {
        let new_matrix = transformation_so_far * node.get_transform();

        if node.index_count > 0 && node.node_type == NodeType::Mesh {
            gl::BindVertexArray(node.vao_id);
            match &self.shader {
                Some(shader) => {
                    let transform_uniform = shader.get_uniform_location("transform");
                    gl::UniformMatrix4fv(transform_uniform, 1, gl::FALSE, new_matrix.as_ptr());
                    let view_uniform = shader.get_uniform_location("view_projection");
                    gl::UniformMatrix4fv(
                        view_uniform,
                        1,
                        gl::FALSE,
                        view_projection_matrix.as_ptr(),
                    );
                }
                None => {}
            }

            gl::DrawElements(
                gl::TRIANGLES,
                node.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        for &child in &node.children {
            self.draw_scene(&*child, view_projection_matrix, &new_matrix);
        }
    }

    fn handle_camera(&mut self, root_node: &mut Pin<&mut SceneNode>, offset: &Vec3) {
        let helicopter = root_node.get_child(HELICOPTER_INDEX);

        self.controls.set_position(helicopter.position + offset);
    }
}

fn helicopter_animation(elapsed: f32, root_node: &mut Pin<&mut SceneNode>) {
    for i in 2..HELI_COUNT + 1 {
        let heading = simple_heading_animation(elapsed + i as f32 * 1.1);
        root_node.get_child(i as usize).position =
            vec3(heading.x, root_node.get_child(1).position.y, heading.z);
        root_node.get_child(i as usize).reference_point =
            vec3(heading.x, root_node.get_child(1).position.y, heading.z);
        root_node.get_child(i as usize).rotation = vec3(heading.pitch, heading.yaw, heading.roll);

        // ROTATE
        //root_node.get_child(1).rotation = self.controls.get_position();
        //root_node.get_child(0).rotation = vec3(0.0, 0.0,elapsed.sin());

        // rotate rotors
        root_node.get_child(i as usize).get_child(1).rotation = vec3(elapsed * 30.0, 0.0, 0.0);
        root_node.get_child(i as usize).get_child(0).rotation = vec3(0.0, elapsed * 30.0, 0.0);
    }
}
