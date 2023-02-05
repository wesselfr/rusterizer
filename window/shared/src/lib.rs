pub mod camera;
pub mod mesh;
pub mod texture;
pub mod transform;
use glam::Vec2;

use crate::camera::*;
use crate::mesh::*;
use crate::texture::*;
use crate::transform::*;

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

pub type FnPtrDraw = fn(u16, u16, u32);

pub struct State {
    pub version: u32,
    pub time_passed: f32,
    pub draw_fn: FnPtrDraw,
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
    pub camera: Camera,
    pub should_clear: bool,
    pub clear_color: u32,
}

impl State {
    pub fn finalize(&self) {
        println!("LIB ACTIVE!");
    }
    pub fn draw(&self, x: u16, y: u16, color: u32) {
        (self.draw_fn)(x, y, color);
    }
    pub fn set_clear_color(&mut self, color: u32) {
        self.clear_color = color;
    }
}

pub fn index_to_coords(p: usize) -> Vec2 {
    Vec2::new(p as f32 / WIDTH as f32, p as f32 % WIDTH as f32)
}

pub fn coords_to_index(coord: Vec2) -> usize {
    coord.x as usize * WIDTH + coord.y as usize
}

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32;
    argb = (argb << 8) + r as u32;
    argb = (argb << 8) + g as u32;
    argb = (argb << 8) + b as u32;
    argb
}
