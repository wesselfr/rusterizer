use glam::{Vec2, Vec3, Vec3Swizzles};

use crate::{barycentric_coordinates, edge_function_cw, index_to_coords, to_argb8, WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

pub fn draw_triangle(v0: Vertex, v1: Vertex, v2: Vertex, buff: &mut Vec<u32>) {
    // Todo: optimize this by only itterating over the region that needs to be updated.
    for (i, pixel) in buff.iter_mut().enumerate() {
        let coords = index_to_coords(i, WIDTH);

        let area = edge_function_cw(v0.position.xy(), v1.position.xy(), v2.position.xy());
        // if m0 > 0.0 && m1 > 0.0 && m2 > 0.0 {
        //     buffer[i] = to_argb8(255, 255, 0, 0);
        // } else {
        //     buffer[i] = to_argb8(255, 0, 0, 0);
        // }
        let bary = barycentric_coordinates(
            coords,
            v0.position.xy(),
            v1.position.xy(),
            v2.position.xy(),
            area,
        );
        if let Some(b) = bary {
            let color = b.x * v0.color + b.y * v1.color + b.z * v2.color;
            *pixel = to_argb8(
                255,
                (255.0 * color.x) as u8,
                (255.0 * color.y) as u8,
                (255.0 * color.z) as u8,
            )
        }
        //buffer[i] = to_argb8(255, m0 as u8, m1 as u8, m2 as u8);
    }
}
