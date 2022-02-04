use glam::{Vec2, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}
