extern crate minifb;

use glam::{Mat2, Vec2};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

pub fn edge_function_cw(v0: Vec2, v1: Vec2, p:Vec2) ->f32{
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn index_to_coords(p: usize, height: usize) -> (usize, usize){
    (p % height, p / height)
}

pub fn to_argb8(a: u8, r: u8, g:u8, b:u8) ->u32{
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

fn main() {
    let vec1 = Vec2::new(20.0, 30.0);
    let vec2 = Vec2::new(30.0, 60.0);

    let matrix = Mat2::from_cols(vec1, vec2);
    let det = matrix.determinant();
    println!("Determinant: {}", det);

    let edge = (Vec2::new(0.0,0.0), Vec2::new(WIDTH as f32, HEIGHT as f32));

    println!("Edge function: {}", edge_function_cw(vec1, vec2, Vec2::new(10.0, 20.0)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..buffer.len() {

            let coords = index_to_coords(i as usize, HEIGHT);
            let coords = Vec2::new(coords.0 as f32, coords.1 as f32);
            let side = edge_function_cw(coords, edge.0, edge.1);
            if side > 0.0{
                buffer[i] = to_argb8(255, 255, 0, 0);
            }
            else{
                buffer[i] = to_argb8(255, 0, 255, 0);
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
