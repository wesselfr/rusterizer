use crate::Transform;
use glam::Mat4;
use std::f32::consts::PI;

pub struct Camera {
    pub near_plane: f32,
    pub far_plane: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub transform: Transform,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            near_plane: 0.1,
            far_plane: 100.0,
            fov: PI / 4.0,
            aspect_ratio: 1.0,
            transform: Transform::IDENTITY,
        }
    }

    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.translation,
            self.transform.translation + self.transform.forward(),
            self.transform.up(),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
