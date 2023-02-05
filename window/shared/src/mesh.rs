use std::ops::{Add, Mul, MulAssign, Sub};

use glam::{UVec3, Vec2, Vec3, Vec4};

use crate::transform::Transform;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self {
            position,
            color,
            uv,
        }
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
        self.color *= rhs;
        self.uv *= rhs;
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self {
            position,
            color,
            uv,
        }
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self {
            position,
            color,
            uv,
        }
    }
}

pub struct Mesh {
    pub triangles: Vec<UVec3>,
    pub vertices: Vec<Vertex>,
    pub transform: Transform,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
            transform: Transform::IDENTITY,
        }
    }

    pub fn triangles(&self) -> &Vec<UVec3> {
        &self.triangles
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn get_triangle_vertices(&self, triangle: UVec3) -> [&Vertex; 3] {
        [
            &self.vertices[triangle.x as usize],
            &self.vertices[triangle.y as usize],
            &self.vertices[triangle.z as usize],
        ]
    }

    pub fn add_vertices(&mut self, triangles: &mut Vec<UVec3>, vertices: &mut Vec<Vertex>) {
        self.triangles.append(triangles);
        self.vertices.append(vertices);
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
