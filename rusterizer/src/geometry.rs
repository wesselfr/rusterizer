use std::ops::{Mul, MulAssign};

use crate::{
    camera::Camera,
    transform::Transform,
    utils::{coords_to_index, map_to_range, to_argb8},
    Texture,
};
use glam::{UVec3, Vec2, Vec3, Vec3Swizzles, Vec4};
use shared::{State, HEIGHT};

use crate::{barycentric_coordinates, edge_function_cw, index_to_coords, WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
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

pub fn draw_triangle(
    vertices: [&Vertex; 3],
    texture: Option<&Texture>,
    transform: &Transform,
    cam: &Camera,
    viewport: Vec2,
    state: &State,
    zbuff: &mut Vec<f32>,
) {
    let mvp = cam.projection() * cam.view() * transform.local();

    let clip0 = mvp * Vec4::from((vertices[0].position, 1.0));
    let clip1 = mvp * Vec4::from((vertices[1].position, 1.0));
    let clip2 = mvp * Vec4::from((vertices[2].position, 1.0));

    let rec0 = 1.0 / clip0.length();
    let rec1 = 1.0 / clip1.length();
    let rec2 = 1.0 / clip2.length();

    // This would be the output of the vertex shader (clip space)
    // then we perform perspective division to transform in ndc
    // now x,y,z componend of ndc are between -1 and 1
    let ndc0 = clip0 * rec0;
    let ndc1 = clip1 * rec1;
    let ndc2 = clip2 * rec2;

    let v0 = *vertices[0] * rec0;
    let v1 = *vertices[1] * rec1;
    let v2 = *vertices[2] * rec2;

    // screeen coordinates remapped to window
    let sc0 = glam::vec2(
        map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport.x),
        map_to_range(ndc0.y, -1.0, 1.0, 0.0, viewport.y),
    );
    let sc1 = glam::vec2(
        map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport.x),
        map_to_range(ndc1.y, -1.0, 1.0, 0.0, viewport.y),
    );
    let sc2 = glam::vec2(
        map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport.x),
        map_to_range(ndc2.y, -1.0, 1.0, 0.0, viewport.y),
    );

    let bounds = BoundingBox2D::get_bounds_from_triangle(&[sc0, sc1, sc2]);

    // Loop over positions instead of pixels, to only update the part of the screen that is needed.
    for y in bounds.min.y as usize..=bounds.max.y as usize {
        for x in bounds.min.x as usize..=bounds.max.x as usize {
            let coords = Vec2::new(x as f32, y as f32);
            let pixel_id = coords_to_index(coords);

            // Ensure pixel is within bounds
            if pixel_id > WIDTH * HEIGHT {
                continue;
            }

            let area = edge_function_cw(sc0, sc1, sc2);
            let bary = barycentric_coordinates(coords, sc0, sc1, sc2, area);
            if let Some(b) = bary {
                let correction = b.x * rec0 + b.y * rec1 + b.z * rec2;
                let correction = 1.0 / correction;
                let depth = b.x * ndc0.z + b.y * ndc1.z + b.z * ndc2.z;

                if depth <= zbuff[pixel_id] {
                    let color: u32;

                    match texture {
                        Some(texture) => {
                            let tex_coords = b.x * v0.uv + b.y * v1.uv + b.z * v2.uv;
                            let tex_coords = tex_coords * correction;
                            color = texture.argb_at_uv(tex_coords.x, tex_coords.y);
                        }
                        None => {
                            let vertex_color = b.x * v0.color + b.y * v1.color + b.z * v2.color;
                            let vertex_color = vertex_color * correction;
                            color = to_argb8(
                                255,
                                (vertex_color.x * 255.0) as u8,
                                (vertex_color.y * 255.0) as u8,
                                (vertex_color.z * 255.0) as u8,
                            );
                        }
                    }
                    state.draw(x as u16, y as u16, color);
                }
            }
        }
    }
}

pub struct Mesh {
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
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

    pub fn draw_mesh(
        &self,
        texture: Option<&Texture>,
        transform: &Transform,
        cam: &Camera,
        viewport: Vec2,
        state: &State,
        zbuff: &mut Vec<f32>,
    ) {
        for triangle in &self.triangles {
            let vertices = self.get_triangle_vertices(*triangle);
            draw_triangle(vertices, texture, transform, cam, viewport, state, zbuff);
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BoundingBox2D {
    pub min: Vec2,
    pub max: Vec2,
}

impl BoundingBox2D {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn get_bounds_from_triangle(positions: &[Vec2; 3]) -> Self {
        let min = Vec2::new(
            positions[0].x.min(positions[1].x.min(positions[2].x)),
            positions[0].y.min(positions[1].y.min(positions[2].y)),
        );
        let max = Vec2::new(
            positions[0].x.max(positions[1].x.max(positions[2].x)),
            positions[0].y.max(positions[1].y.max(positions[2].y)),
        );

        Self { min, max }
    }
}
