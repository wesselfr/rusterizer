use std::f32::consts::PI;
use std::f32::INFINITY;
use std::path::Path;

use glam::Quat;
use glam::UVec3;
use glam::Vec2;
use glam::Vec3;
use glam::Vec3Swizzles;
use glam::Vec4;
use shared::camera::Camera;
use shared::mesh::Mesh;
use shared::mesh::Vertex;
use shared::texture::Texture;
use shared::transform::Transform;
use shared::*;

pub mod color;
use crate::color::*;

pub mod utils;
use crate::utils::to_argb8;
use crate::utils::*;

pub mod geometry;
use crate::geometry::*;

fn load_gltf_mesh(path: &Path) -> Option<Mesh> {
    println!("Loading GLTF: {:?}", path);
    let result = gltf::import(path);

    match result {
        Ok((gltf, buffers, _)) => {
            for mesh in gltf.meshes() {
                let mut positions: Vec<Vec3> = Vec::new();
                let mut tex_coords: Vec<Vec2> = Vec::new();
                let mut raw_indices = vec![];

                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                    if let Some(indices_reader) = reader.read_indices() {
                        indices_reader.into_u32().for_each(|i| raw_indices.push(i));
                    }
                    if let Some(positions_reader) = reader.read_positions() {
                        positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
                    }
                    if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                        tex_coord_reader
                            .into_f32()
                            .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
                    }

                    println!("Num indices: {:?}", raw_indices.len());
                    println!("tex_coords: {:?}", tex_coords.len());
                    println!("positions: {:?}", positions.len());
                }

                let mut vertices: Vec<Vertex> = Vec::new();
                vertices.reserve(positions.len());
                for i in 0..positions.len() {
                    vertices.push(Vertex {
                        position: positions[i].extend(1.0),
                        color: Vec3::ONE,
                        uv: tex_coords[i],
                    })
                }

                let mut triangles: Vec<UVec3> = raw_indices
                    .chunks_exact(3)
                    .map(|tri| UVec3::new(tri[0], tri[1], tri[2]))
                    .collect();

                let mut out_mesh = Mesh::new();
                out_mesh.add_vertices(&mut triangles, &mut vertices);
                return Some(out_mesh);
            }
        }
        Err(e) => {
            println!("Error while loading gltf model: {:?}", e);
            return None;
        }
    }
    println!("No meshes found in gltf.");
    None
}

pub fn draw_grid(
    state: &RenderState,
    vertices: [&Vertex; 3],
    bary_centric: Vec3,
    correction: f32,
) -> u32 {
    let v0 = vertices[0];
    let v1 = vertices[1];
    let v2 = vertices[2];

    match state.texture {
        Some(texture) => {
            let tex_coords =
                bary_centric.x * v0.uv + bary_centric.y * v1.uv + bary_centric.z * v2.uv;
            let mut tex_coords = tex_coords * correction;

            tex_coords.x -= state.variables["time_passed"] * 0.3;

            let col = texture.argb_at_uv(tex_coords.x, tex_coords.y);
            let mut col = Color::from_argb8(col);
            let alpha = col.a as f32 / 255.0;
            col.r = lerp(state.clear_color.r as f32, col.r as f32, alpha) as u8;
            col.g = lerp(state.clear_color.g as f32, col.g as f32, alpha) as u8;
            col.b = lerp(state.clear_color.b as f32, col.b as f32, alpha) as u8;

            col.to_argb8()
        }
        None => {
            let vertex_color =
                bary_centric.x * v0.color + bary_centric.y * v1.color + bary_centric.z * v2.color;
            let vertex_color = vertex_color * correction;
            to_argb8(
                255,
                (vertex_color.x * 255.0) as u8,
                (vertex_color.y * 255.0) as u8,
                (vertex_color.z * 255.0) as u8,
            )
        }
    }
}
#[no_mangle]
pub fn setup(shared_state: &mut State) {
    println!("Application version: {}", shared_state.version);

    shared_state.textures.clear();
    let texture = Texture::load(Path::new("assets/synthwave/sun.png"));
    if let Ok(texture) = texture {
        shared_state.textures.push(texture);
    }

    //let texture = Texture::load(Path::new("assets/test.jpg"));
    let texture = Texture::load(Path::new("assets/synthwave/grid.jpg"));
    if let Ok(texture) = texture {
        shared_state.textures.push(texture);
    }

    // Clear previous loaded meshes
    shared_state.meshes.clear();

    // Floor
    let mut mesh = Mesh::new();
    let mut vertices = vec![
        Vertex {
            position: Vec4::new(-1.0, -1.0, 0.0, 1.0),
            color: Vec3::new(1.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
            color: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, 1.0, 0.0, 1.0),
            color: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, -1.0, 0.0, 1.0),
            color: Vec3::new(1.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 0.0),
        },
    ];
    let mut indices = vec![UVec3::new(2, 1, 0), UVec3::new(3, 2, 0)];
    mesh.add_vertices(&mut indices, &mut vertices);
    mesh.transform = Transform::from_translation_rotation(
        Vec3::new(2.0, 0.0, -12.5),
        Quat::from_rotation_y(-89.0 * (PI / 180.0)),
    );
    mesh.transform.scale = Vec3::ONE * 12.0;
    shared_state.meshes.push(mesh);

    // Sun
    let mut mesh = Mesh::new();
    let mut vertices = vec![
        Vertex {
            position: Vec4::new(-1.0, -1.0, 0.0, 1.0),
            color: Vec3::new(1.0, 0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
            color: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, 1.0, 0.0, 1.0),
            color: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, -1.0, 0.0, 1.0),
            color: Vec3::new(1.0, 0.0, 1.0),
            uv: Vec2::new(1.0, 0.0),
        },
    ];
    let mut indices = vec![UVec3::new(2, 1, 0), UVec3::new(3, 2, 0)];
    mesh.transform = Transform::from_translation(Vec3::new(-2.0, 0.0, -30.0));
    mesh.transform.scale = Vec3::ONE * 8.0;
    mesh.add_vertices(&mut indices, &mut vertices);
    shared_state.meshes.push(mesh);

    shared_state.should_clear = true;

    shared_state.finalize();
}

#[no_mangle]
pub fn update(shared_state: &mut State) {
    let mut z_buffer = vec![INFINITY; WIDTH * HEIGHT];

    shared_state.camera.transform = Transform::from_translation(Vec3::new(
        1.0 + shared_state.time_passed.sin() * 0.5,
        0.0,
        0.0,
    ));

    let render_state_sun =
        RenderState::from_shade_fn(shared_state, draw_texture, Some(&shared_state.textures[0]));

    let mut render_state_grid =
        RenderState::from_shade_fn(shared_state, draw_grid, Some(&shared_state.textures[1]));
    render_state_grid
        .variables
        .insert("time_passed", shared_state.time_passed);

    let grid_mesh = RenderMesh::from_mesh(&shared_state.meshes[0]);
    grid_mesh.draw_mesh(
        &render_state_grid,
        &shared_state.camera,
        Vec2::new(WIDTH as f32, HEIGHT as f32),
        &mut z_buffer,
    );

    let sun_mesh = RenderMesh::from_mesh(&shared_state.meshes[1]);
    sun_mesh.draw_mesh(
        &render_state_sun,
        &shared_state.camera,
        Vec2::new(WIDTH as f32, HEIGHT as f32),
        &mut z_buffer,
    );
    shared_state.set_clear_color(0xff110012);
}
