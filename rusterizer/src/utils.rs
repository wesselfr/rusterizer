use glam::{Vec2, Vec3};
use shared::State;

use crate::color::Color;

fn plotline_low(v0: Vec2, v1: Vec2, color: Color, state: &State) {
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
        state.draw(x as u16, y as u16, color.to_argb8());
        if d >= 0.0 {
            y += yi;
            d += 2.0 * (dy - dx);
        } else {
            d += 2.0 * dy
        }
    }
}

fn plotline_high(v0: Vec2, v1: Vec2, color: Color, state: &State) {
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
        state.draw(x as u16, y as u16, color.to_argb8());
        if d >= 0.0 {
            x += xi;
            d += 2.0 * (dx - dy);
        } else {
            d += 2.0 * dx;
        }
    }
}

// Bresenham's line algorithm
pub fn plotline(v0: Vec2, v1: Vec2, color: Color, state: &State) {
    if (v1.y - v0.y).abs() < (v1.x - v0.x).abs() {
        if v0.x > v1.x {
            plotline_low(v1, v0, color, state);
        } else {
            plotline_low(v0, v1, color, state);
        }
    } else {
        if v0.y > v1.y {
            plotline_high(v1, v0, color, state);
        } else {
            plotline_high(v0, v1, color, state);
        }
    }
}

pub fn edge_function_cw(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}

pub fn barycentric_coordinates(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    area: f32,
) -> Option<Vec3> {
    let m0 = edge_function_cw(point, v1, v2);
    let m1 = edge_function_cw(point, v2, v0);
    let m2 = edge_function_cw(point, v0, v1);

    let a = 1.0 / area;
    if m0 > 0.0 && m1 > 0.0 && m2 > 0.0 {
        Some(Vec3::new(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}

pub fn lerp<T>(min: T, max: T, t: f32) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    min + (max - min) * t
}

pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}
