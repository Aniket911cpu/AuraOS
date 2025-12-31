// core/window_manager.rs
use crate::window::{Window, WindowType};
use crate::animator::AnimationState;
use cgmath::Vector3;
use std::collections::HashMap;

pub struct WindowManager {
    pub windows: HashMap<u64, Window>,
    pub next_id: u64,
    pub active_window_id: Option<u64>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            next_id: 1,
            active_window_id: None,
        }
    }

    pub fn spawn_window(&mut self, title: String, w_type: WindowType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut win = Window::new(id, title, w_type);
        
        // Initialize windows at the center (0,0) but slightly "off screen" in Z for entry anim?
        // For now, just spawn at 0,0,0
        win.position_anim = AnimationState::new(Vector3::new(0.0, 0.0, 0.0));
        
        self.windows.insert(id, win);
        self.focus_window(id);
        
        id
    }

    pub fn close_window(&mut self, id: u64) {
        self.windows.remove(&id);
        if self.active_window_id == Some(id) {
            self.active_window_id = None;
            // TODO: Focus next available window
        }
    }

    pub fn focus_window(&mut self, id: u64) {
        // Lower old active window
        if let Some(old_id) = self.active_window_id {
            if let Some(old_win) = self.windows.get_mut(&old_id) {
                old_win.set_elevation(1.0); // Resting height
            }
        }

        // Raise new active window
        if let Some(new_win) = self.windows.get_mut(&id) {
            new_win.set_elevation(5.0); // Active height
            self.active_window_id = Some(id);
        }
    }

    pub fn move_window(&mut self, id: u64, target: Vector3<f32>) {
        if let Some(win) = self.windows.get_mut(&id) {
            win.move_to(target);
        }
    }

    pub fn update(&mut self) {
        for win in self.windows.values_mut() {
            win.update_physics();
        }
    }
}
