extern crate minifb;

use glam::Vec2;
use minifb::{Key, Window, WindowOptions};

pub mod color;
pub use color::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;
const DEBUG_COLOR: Color = Color {
    a: 255,
    r: 255,
    g: 0,
    b: 255,
};

pub fn edge_function_cw(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn index_to_coords(p: usize, height: usize) -> (usize, usize) {
    (p / height, p % height)
}

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

fn plotline_low(v0: Vec2, v1: Vec2, color: Color, buff: &mut Vec<u32>) {
    let dx = v1.x - v0.x;
    let mut dy = v1.y - v0.y;
    let mut yi = 1.0;
    if dy < 0.0 {
        yi = -1.0;
        dy = -dy;
    }
    let mut d = (2.0 * dy) - dx;
    let mut y = v0.y;

    for x in v0.x as usize..v1.x as usize {
        buff[x * WIDTH + y as usize] = color.to_argb8();
        if d >= 0.0 {
            y += yi;
            d += 2.0 * (dy - dx);
        } else {
            d += 2.0 * dy
        }
    }
}

fn plotline_high(v0: Vec2, v1: Vec2, color: Color, buff: &mut Vec<u32>) {
    let mut dx = v1.x - v0.x;
    let dy = v1.y - v0.y;
    let mut xi = 1.0;
    if dx < 0.0 {
        xi = -1.0;
        dx = -dx;
    }
    let mut d = (2.0 * dx) - dy;
    let mut x = v0.x;

    for y in v0.y as usize..v1.y as usize {
        buff[x as usize * WIDTH + y] = color.to_argb8();
        if d >= 0.0 {
            x += xi;
            d += 2.0 * (dx - dy);
        } else {
            d += 2.0 * dx;
        }
    }
}

fn plotline(v0: Vec2, v1: Vec2, color: Color, buff: &mut Vec<u32>) {
    if (v1.y - v0.y).abs() < (v1.x - v0.x).abs() {
        if v0.x > v1.x {
            plotline_low(v1, v0, color, buff);
        } else {
            plotline_low(v0, v1, color, buff);
        }
    } else {
        if v0.y > v1.y {
            plotline_high(v1, v0, color, buff);
        } else {
            plotline_high(v0, v1, color, buff);
        }
    }
}

fn main() {
    let triangle = [
        Vec2::new(100.0, 100.0),
        Vec2::new(250.0, 400.0),
        Vec2::new(400.0, 100.0),
    ];

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

    // Limit to max ~120 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(8300)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..buffer.len() {
            let coords = index_to_coords(i as usize, WIDTH);
            let coords = Vec2::new(coords.0 as f32, coords.1 as f32);
            let m0 = edge_function_cw(coords, triangle[1], triangle[2]);
            let m1 = edge_function_cw(coords, triangle[2], triangle[0]);
            let m2 = edge_function_cw(coords, triangle[0], triangle[1]);

            if m0 > 0.0 && m1 > 0.0 && m2 > 0.0 {
                buffer[i] = to_argb8(255, 255, 0, 0);
            } else {
                buffer[i] = to_argb8(255, 0, 0, 0);
            }

            //buffer[i] = to_argb8(255, m0 as u8, m1 as u8, m2 as u8);

            plotline(triangle[0], triangle[1], DEBUG_COLOR, &mut buffer);
            plotline(triangle[1], triangle[2], DEBUG_COLOR, &mut buffer);
            plotline(triangle[2], triangle[0], DEBUG_COLOR, &mut buffer);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
