use std::f32::INFINITY;
use std::path::Path;

use glam::Quat;
use glam::UVec3;
use glam::Vec2;
use glam::Vec3;
use glam::Vec3Swizzles;
use glam::Vec4;
use shared::mesh::Mesh;
use shared::mesh::Vertex;
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
pub fn setup(shared_state: &mut State) {
    println!("Application version: {}", shared_state.version);

    shared_state.textures.clear();
    let texture = Texture::load(Path::new("assets/test.jpg"));
    if let Ok(texture) = texture {
        shared_state.textures.push(texture);
    }

    let mut vertices = vec![
        Vertex {
            position: Vec4::new(-1.0, -1.0, 1.0, 1.0),
            color: Vec3::new(1.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec4::new(-1.0, 1.0, 1.0, 1.0),
            color: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, 1.0, 1.0, 1.0),
            color: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, -1.0, 1.0, 1.0),
            color: Vec3::new(1.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 0.0),
        },
    ];
    let mut indices = vec![UVec3::new(2, 1, 0), UVec3::new(3, 2, 0)];

    let mut mesh = Mesh::new();
    mesh.add_vertices(&mut indices, &mut vertices);
    shared_state.meshes.push(mesh);

    shared_state.should_clear = true;

    shared_state.finalize();
}

#[no_mangle]
pub fn update(shared_state: &mut State) {
    let mut z_buffer = vec![INFINITY; WIDTH * HEIGHT];

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(
            0.0 + shared_state.time_passed.sin() * 0.8,
            0.0,
            2.0 + shared_state.time_passed.cos() * 0.8,
        )),
        ..Default::default()
    };

    camera.transform.rotation = Quat::from_rotation_y(shared_state.time_passed.sin() * 0.5)
        + Quat::from_rotation_x(shared_state.time_passed.cos() * 0.5);

    let render_state =
        RenderState::from_shade_fn(shared_state, draw_texture, Some(&shared_state.textures[0]));

    for mesh in &shared_state.meshes {
        let render_mesh = RenderMesh::from_mesh(mesh);
        render_mesh.draw_mesh(
            &render_state,
            &camera,
            Vec2::new(WIDTH as f32, HEIGHT as f32),
            &mut z_buffer,
        );
    }

    shared_state.set_clear_color(0xff103030);
}
