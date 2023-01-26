use crate::Texture;
use glam::{Vec2, Vec3, Vec3Swizzles};

use crate::{barycentric_coordinates, edge_function_cw, index_to_coords, WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

pub fn draw_triangle(
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    texture: &Texture,
    buff: &mut Vec<u32>,
    zbuff: &mut Vec<f32>,
) {
    // Todo: optimize this by only itterating over the region that needs to be updated.
    // Loop over positions instead of pixels, to only update the part of the screen that is needed.
    for (i, pixel) in buff.iter_mut().enumerate() {
        let coords = index_to_coords(i, WIDTH);

        let area = edge_function_cw(v0.position.xy(), v1.position.xy(), v2.position.xy());
        let bary = barycentric_coordinates(
            coords,
            v0.position.xy(),
            v1.position.xy(),
            v2.position.xy(),
            area,
        );
        if let Some(b) = bary {
            let depth = b.x * v0.position.z + b.y * v1.position.z + b.z + v2.position.z;

            if depth <= zbuff[i] {
                let tex_coords = b.x * v0.uv + b.y * v1.uv + b.z * v2.uv;
                let color = texture.argb_at_uv(tex_coords.x, tex_coords.y);
                *pixel = color;
            }
        }
    }
}
