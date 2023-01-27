use std::f32::INFINITY;
use std::path::Path;

use glam::UVec3;
use glam::Vec2;
use glam::Vec3;
use glam::Vec3Swizzles;
use shared::texture::Texture;
use shared::*;

pub mod color;
use crate::color::*;

pub mod utils;
use crate::utils::*;

pub mod transform;
use crate::transform::*;

pub mod camera;
use crate::camera::*;

pub mod geometry;
use crate::geometry::*;

#[no_mangle]
pub fn setup(test: &mut State) {
    println!("Application version: {}", test.version);

    test.textures.clear();
    let texture = Texture::load(Path::new("assets/test.jpg"));
    if let Ok(texture) = texture {
        test.textures.push(texture);
    }

    test.finalize();
}

#[no_mangle]
pub fn update(shared_state: &mut State) {
    let quad_pos = Vec2::new(0.0, shared_state.time_passed * 0.0);

    let mut z_buffer = vec![INFINITY; WIDTH * HEIGHT];

    let mut vertices = vec![
        Vertex {
            position: Vec3::new(100.0, 100.0, 1.0),
            color: Vec3::new(1.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(100.0, 400.0, 1.0),
            color: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(400.0, 400.0, 1.0),
            color: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(400.0, 100.0, 1.0),
            color: Vec3::new(1.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 0.0),
        },
    ];
    let mut indices = vec![UVec3::new(0, 1, 2), UVec3::new(0, 2, 3)];

    let mut mesh = Mesh::new();
    mesh.add_vertices(&mut indices, &mut vertices);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(
            200.0 + shared_state.time_passed.sin() * 200.0,
            100.0,
            400.0 + shared_state.time_passed.cos() * 200.0,
        )),
        far_plane: 100.0,
        ..Default::default()
    };
    let mut transform = Transform::IDENTITY;

    mesh.draw_mesh(
        Some(&shared_state.textures[0]),
        &transform,
        &camera,
        Vec2::new(WIDTH as f32, HEIGHT as f32),
        shared_state,
        &mut z_buffer,
    );

    shared_state.set_clear_color(0xff103030);
}
