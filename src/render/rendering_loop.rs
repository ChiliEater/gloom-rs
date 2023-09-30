extern crate nalgebra_glm as glm;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::{mem, os::raw::c_void, ptr};

use crate::input::controls::Controls;
use crate::shader::Shader;
use crate::toolbox::{rotate_all, scale_around, rotate_around};
use crate::{shader, util, INITIAL_SCREEN_H, INITIAL_SCREEN_W};
use glm::{pi, vec3, Mat4x4, half_pi, cos};
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
use super::scene_graph::{self, Node, SceneNode};
use super::vao::create_vao;
use super::window_locks::WindowLocks;

const TERRAIN: &str = "./resources/lunarsurface.obj";
const HELICOPTER: &str = "./resources/helicopter.obj";
const COLORCUBE: &str = "./resources/cube.obj";

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

        let mut root_node = self.setup_scene();
        let terrain: &mut SceneNode = root_node.get_child(0);
        let helicopter: &mut SceneNode = root_node.get_child(1);
        let heli_main_rotor: &mut SceneNode = helicopter.get_child(0);
        let heli_tail_rotor: &mut SceneNode = helicopter.get_child(1);
        let heli_door: &mut SceneNode = helicopter.get_child(2);

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
            
            // CURSED HELICOPTER
            //heli_tail_rotor.rotation = vec3(elapsed,0.0,0.0); 

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
                glm::perspective(self.window_aspect_ratio, glm::half_pi(), 0.25, 2000.0);

            unsafe {
                // Clear the color and depth buffers
                //gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::ClearColor(0.0078, 0.302, 0.251, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                // == // Issue the necessary gl:: commands to draw your scene here
                let movement = self.controls.handle(delta_time);
                self.draw_scene_initial(
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

        let mut root_node: Node = SceneNode::new();

        // Add terrain as a child of root.
        root_node.add_child(&SceneNode::from_vao(
            unsafe { create_vao(&terrain_mesh) },
            terrain_mesh.index_count,
        ));

        // Add helicopter body
        let mut helicopter_body_node = SceneNode::from_vao(
            unsafe { create_vao(&helicopter_mesh.body) },
            helicopter_mesh.body.index_count,
        );
        // Try some transformations on helicopter
        helicopter_body_node.scale = vec3(1.0, 1.0, 1.0) * 10.0;
        helicopter_body_node.position = vec3(0.0, 10.0, -20.0);

        // Add helicopter as a child of root.
        root_node.add_child(&helicopter_body_node);

        // Add the rest of helicopter parts as children of body
        for i in 1..4 {
            helicopter_body_node.add_child(&SceneNode::from_vao(
                unsafe { create_vao(&helicopter_mesh[i]) },
                helicopter_mesh[i].index_count,
            ));
        }

        helicopter_body_node.get_child(1).reference_point = vec3(0.35,2.3,10.5)*1.0;
        helicopter_body_node.get_child(1).rotation = vec3(0.0,0.0,0.0);
        root_node
    }

    unsafe fn draw_scene_initial(
        &self,
        node: &Node,
        view_projection_matrix: &Mat4x4,
        transformation_so_far: &Mat4x4,
    ) {
        let new_reference_point = (transformation_so_far*glm::vec4(node.reference_point.x,node.reference_point.y,node.reference_point.z,1.0)).xyz();
        let new_matrix =
        rotate_around(&node.rotation, &new_reference_point)
        * scale_around(&node.scale, &new_reference_point) 
        * glm::translation(&node.position)
        * transformation_so_far
        ;

        for &child in &node.children {
            self.draw_scene(&*child, view_projection_matrix, &new_matrix);
        }
    }

    unsafe fn draw_scene(
        &self,
        node: &scene_graph::SceneNode,
        view_projection_matrix: &Mat4x4,
        transformation_so_far: &Mat4x4,
    ) {
        let new_reference_point = (transformation_so_far*glm::vec4(node.reference_point.x,node.reference_point.y,node.reference_point.z,1.0)).xyz();
        let new_matrix =
        rotate_around(&node.rotation, &new_reference_point)
        * scale_around(&node.scale, &new_reference_point) 
        * glm::translation(&node.position)
        * transformation_so_far
        ;

        if node.index_count > 0 {

            gl::BindVertexArray(node.vao_id);
            match &self.shader {
                Some(shader) => {
                    let position_uniform = shader.get_uniform_location("transform");
                    gl::UniformMatrix4fv(
                        position_uniform,
                        1,
                        gl::FALSE,
                        (view_projection_matrix * new_matrix).as_ptr(),
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
}
