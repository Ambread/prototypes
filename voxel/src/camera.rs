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

    const MOVE_SPEED: f32 = 0.05;
    const ROTATE_SPEED: Rad<f32> = Rad(std::f32::consts::PI / 64.0);
    const VERTICAL_ROTATE_LIMIT: Rad<f32> = Rad(std::f32::consts::FRAC_PI_2);

    pub fn update(&mut self, pressed_keys: &HashSet<Key>) {
        if pressed_keys.contains(&Key::R) {
            *self = Self::new();
        }

        self.update_rotate(pressed_keys);
        self.update_movement(pressed_keys);
    }

    fn update_rotate(&mut self, pressed_keys: &HashSet<Key>) {
        if pressed_keys.contains(&Key::Up) {
            self.rotation.x -= Self::ROTATE_SPEED;
        }
        if pressed_keys.contains(&Key::Down) {
            self.rotation.x += Self::ROTATE_SPEED;
        }

        if pressed_keys.contains(&Key::Left) {
            self.rotation.y -= Self::ROTATE_SPEED;
        }
        if pressed_keys.contains(&Key::Right) {
            self.rotation.y += Self::ROTATE_SPEED;
        }

        self.rotation.x = Rad(self.rotation.x.0.clamp(
            -Self::VERTICAL_ROTATE_LIMIT.0,
            Self::VERTICAL_ROTATE_LIMIT.0,
        ));
    }

    fn update_movement(&mut self, pressed_keys: &HashSet<Key>) {
        let mut movement = Vector3::new(0.0, 0.0, 0.0);

        if pressed_keys.contains(&Key::W) {
            movement.z += self.rotation.y.cos();
            movement.x -= self.rotation.y.sin();
        }
        if pressed_keys.contains(&Key::S) {
            movement.z -= self.rotation.y.cos();
            movement.x += self.rotation.y.sin();
        }

        if pressed_keys.contains(&Key::A) {
            movement.z += self.rotation.y.sin();
            movement.x += self.rotation.y.cos();
        }
        if pressed_keys.contains(&Key::D) {
            movement.z -= self.rotation.y.sin();
            movement.x -= self.rotation.y.cos();
        }

        if pressed_keys.contains(&Key::Space) {
            movement.y -= 1.0;
        }
        if pressed_keys.contains(&Key::LeftShift) {
            movement.y += 1.0;
        }

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
