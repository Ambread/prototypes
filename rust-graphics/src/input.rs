use glfw::{Action, Key, WindowEvent};
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Input {
    held: HashSet<Key>,
    pressed: HashSet<Key>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(key, _, Action::Press, _) => {
                self.held.insert(*key);
                self.pressed.insert(*key);
            }

            WindowEvent::Key(key, _, Action::Release, _) => {
                self.held.remove(key);
            }

            _ => {}
        }
    }

    pub fn is_held(&self, key: Key) -> bool {
        self.held.contains(&key)
    }

    pub fn has_pressed(&mut self, key: Key) -> bool {
        self.pressed.take(&key).is_some()
    }
}
