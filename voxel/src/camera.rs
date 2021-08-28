use cgmath::{perspective, Angle, Deg, InnerSpace, Matrix4, Rad, Vector2, Vector3, Zero};
use glfw::Key;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector2<Rad<f32>>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector2::new(Rad(0.0), Rad(0.0)),
        }
    }

    const MOVE_SPEED: f32 = 0.1;
    // Ratio of mouse movement to rads, I just found a number that works
    const ROTATE_SENSITIVITY: f64 = 1000.0;
    // Prevent from going past "all the way up/down" as to not flip camera
    const VERTICAL_ROTATE_LIMIT: Rad<f32> = Rad(std::f32::consts::FRAC_PI_2);

    pub fn update(&mut self, cursor_position: &Vector2<f64>, pressed_keys: &HashSet<Key>) {
        if pressed_keys.contains(&Key::R) {
            *self = Self::new();
        }

        self.update_rotate(cursor_position);
        self.update_movement(pressed_keys);
    }

    fn update_rotate(&mut self, cursor_position: &Vector2<f64>) {
        // Map cursor_position to Rads, arbitrarily using magic constant
        let cursor_position = cursor_position.map(|it| {
            let it = it / Self::ROTATE_SENSITIVITY;
            Rad(it as f32)
        });

        // Yes the components are swapped for some reason
        self.rotation.x += cursor_position.y;
        self.rotation.y += cursor_position.x;

        // Apply VERTICAL_ROTATE_LIMIT, needs ceremony due to Rad
        self.rotation.x = Rad(self.rotation.x.0.clamp(
            -Self::VERTICAL_ROTATE_LIMIT.0,
            Self::VERTICAL_ROTATE_LIMIT.0,
        ));
    }

    fn update_movement(&mut self, pressed_keys: &HashSet<Key>) {
        // We need a temp movement vector to normalize it later, to prevent diagonals going faster
        let mut movement = Vector3::new(0.0, 0.0, 0.0);

        // I can't explain these, I just flipped things until it worked
        // There's likely some fancy vector math I can't grasp

        // Forwards and backwards
        if pressed_keys.contains(&Key::W) {
            movement.z += self.rotation.y.cos();
            movement.x -= self.rotation.y.sin();
        }
        if pressed_keys.contains(&Key::S) {
            movement.z -= self.rotation.y.cos();
            movement.x += self.rotation.y.sin();
        }

        // Left and right
        if pressed_keys.contains(&Key::A) {
            movement.z += self.rotation.y.sin();
            movement.x += self.rotation.y.cos();
        }
        if pressed_keys.contains(&Key::D) {
            movement.z -= self.rotation.y.sin();
            movement.x -= self.rotation.y.cos();
        }

        // Up and down, yeah this one is simple
        if pressed_keys.contains(&Key::Space) {
            movement.y -= 1.0;
        }
        if pressed_keys.contains(&Key::LeftShift) {
            movement.y += 1.0;
        }

        // Normalizing a zero vector breaks everything
        if !movement.is_zero() {
            self.position += movement.normalize() * Self::MOVE_SPEED;
        }
    }

    const FOV: Deg<f32> = Deg(90.0);
    const Z_NEAR: f32 = 0.1;
    const Z_FAR: f32 = 100.0;

    pub fn get_projection(&self, window_size: &Vector2<i32>) -> Matrix4<f32> {
        let aspect = window_size.x as f32 / window_size.y as f32;

        perspective(Self::FOV, aspect, Self::Z_NEAR, Self::Z_FAR)
            * Matrix4::from_angle_x(self.rotation.x)
            * Matrix4::from_angle_y(self.rotation.y)
            * Matrix4::from_translation(self.position)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
