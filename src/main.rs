extern crate minifb;

use std::path::Path;

use geometry::draw_triangle;
use glam::{Vec2, Vec3, Vec3Swizzles};
use minifb::{Key, Window, WindowOptions};

pub mod color;
pub use color::*;

pub mod utils;
pub use utils::*;

pub mod texture;
pub use texture::*;

pub mod geometry;
pub use geometry::Vertex;

const DEBUG_COLOR: Color = Color {
    a: 255,
    r: 255,
    g: 0,
    b: 255,
};

fn main() {
    let quad = [
        Vertex {
            position: Vec3::new(100.0, 100.0, 1.0),
            color: Vec3::new(1.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(100.0, 612.0, 1.0),
            color: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(612.0, 612.0, 1.0),
            color: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(1.0,1.0)
        },
        Vertex {
            position: Vec3::new(612.0, 100.0, 1.0),
            color: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 0.0),
        },
    ];

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut z_buffer: Vec<f32> = vec![100.0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~120 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(8300)));

    let texture = Texture::load(Path::new("assets/test.jpg"));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_triangle(
            quad[0],
            quad[1],
            quad[2],
            &texture,
            &mut buffer,
            &mut z_buffer,
        );
        draw_triangle(
            quad[0],
            quad[2],
            quad[3],
            &texture,
            &mut buffer,
            &mut z_buffer,
        );

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
