use glutin::{event::VirtualKeyCode, ContextWrapper, NotCurrent, window::Window};
use std::sync::{Arc, Mutex};

pub struct WindowLocks {
    window_size: Arc<Mutex<(u32, u32, bool)>>,
    pressed_keys: Arc<Mutex<Vec<VirtualKeyCode>>>,
    mouse_delta: Arc<Mutex<(f32, f32)>>,
}

impl WindowLocks {
    pub fn new(initial_screen_w: u32, initial_screen_h: u32) -> WindowLocks {
        WindowLocks {
            window_size: Arc::new(Mutex::new((initial_screen_w, initial_screen_h, false))),
            pressed_keys: Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10))),
            mouse_delta: Arc::new(Mutex::new((0f32, 0f32))),
        }
    }

    pub fn window_size(&self) -> Arc<Mutex<(u32, u32, bool)>> {
        Arc::clone(&self.window_size)
    }

    pub fn pressed_keys(&self) -> Arc<Mutex<Vec<VirtualKeyCode>>> {
        Arc::clone(&self.pressed_keys)
    }

    pub fn mouse_delta(&self) -> Arc<Mutex<(f32, f32)>> {
        Arc::clone(&self.mouse_delta)
    }
}
