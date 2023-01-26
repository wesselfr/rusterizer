use glam::Vec2;
use glam::Vec3;
use glam::Vec3Swizzles;
use shared::State;
use shared::HEIGHT;
use shared::WIDTH;

pub mod color;
use crate::color::*;

pub mod utils;
use crate::utils::*;

pub mod texture;
use crate::texture::*;

pub mod geometry;
use crate::geometry::*;

#[no_mangle]
pub fn setup(test: &State) {
    println!("Application version: {}", test.version);
    test.finalize();
}

#[no_mangle]
pub fn update(test: &mut State) {
    test.draw(0, 0, 0x00000000);

    let vertices = [
        Vertex {
            position: Vec3::new(400.0, 100.0, 1.0),
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
            color: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(1.0, 1.0),
        },
    ];

    for i in 0..WIDTH * HEIGHT {
        let pos = index_to_coords(i);

        let area = edge_function_cw(vertices[0].position.xy(), vertices[1].position.xy(), vertices[2].position.xy());
        let bary = barycentric_coordinates(
            pos,
            vertices[0].position.xy(),
            vertices[1].position.xy(),
            vertices[2].position.xy(),
            area,
        );

        if let Some(b) = bary {
            test.draw(pos.x as u16, pos.y as u16, 0xffff30ff)
        }
    }

    plotline(
        vertices[0].position.xy(),
        vertices[1].position.xy(),
        Color {
            a: 255,
            r: 255,
            g: 255,
            b: 255,
        },
        &test,
    );

    plotline(
        vertices[1].position.xy(),
        vertices[2].position.xy(),
        Color {
            a: 255,
            r: 255,
            g: 255,
            b: 255,
        },
        &test,
    );

    plotline(
        vertices[2].position.xy(),
        vertices[0].position.xy(),
        Color {
            a: 255,
            r: 255,
            g: 255,
            b: 255,
        },
        &test,
    );

    test.set_clear_color(0xff103030);
    //test.draw_text(0, 9, &format!("Current Version: {}", test.version));
    //test.draw_text(0, 10, "Custom draw function working.");
    //test.draw_text(0, 12, "Roguelike");
}
