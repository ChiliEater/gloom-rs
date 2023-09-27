use crate::render::window_locks::{WindowLocks, self};
use glutin::{event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
}, platform::run_return::EventLoopExtRunReturn};
use glutin::event_loop::ControlFlow;
use std::sync::{Arc, Mutex, RwLock};

pub struct InputLoop {
    render_thread_healthy: Arc<RwLock<bool>>,
    window_size: Arc<Mutex<(u32, u32, bool)>>,
    pressed_keys: Arc<Mutex<Vec<VirtualKeyCode>>>,
    mouse_delta: Arc<Mutex<(f32, f32)>>,
}

impl InputLoop {
    pub fn new(render_thread_healthy: Arc<RwLock<bool>>, window_locks: &WindowLocks) -> InputLoop {
        InputLoop {
            render_thread_healthy,
            window_size: window_locks.window_size(),
            pressed_keys: window_locks.pressed_keys(),
            mouse_delta: window_locks.mouse_delta(),
        }
    }

    pub fn start(&self, event_loop: &mut glutin::event_loop::EventLoop<()>) {
        // Start the event loop -- This is where window events are initially handled
        let render_thread_healthy = Arc::clone(&self.render_thread_healthy);
        let window_size = Arc::clone(&self.window_size);
        let mouse_delta = Arc::clone(&self.mouse_delta);
        let pressed_keys = Arc::clone(&self.pressed_keys);
        event_loop.run_return(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            // Terminate program if render thread panics
            if let Ok(health) = render_thread_healthy.read() {
                if !*health {
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
                    if let Ok(mut new_size) = window_size.lock() {
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
                    if let Ok(mut keys) = pressed_keys.lock() {
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
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    // Accumulate mouse movement
                    if let Ok(mut position) = mouse_delta.lock() {
                        *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                    }
                }
                _ => {}
            }
        });
    }
}

