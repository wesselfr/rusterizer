use glam::Vec2;
use stb_image;
use std::path::Path;

use crate::to_argb8;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub depth: usize,
}

impl Texture {
    pub fn load(path: &Path) -> Result<Self, &'static str> {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {
            let data;
            if image.depth == 3 {
                data = (0..image.data.len() / 3)
                    .map(|id| {
                        to_argb8(
                            255,
                            image.data[id * 3],
                            image.data[id * 3 + 1],
                            image.data[id * 3 + 2],
                        )
                    })
                    .collect();
            } else if image.depth == 4 {
                data = (0..image.data.len() / 4)
                    .map(|id| {
                        to_argb8(
                            image.data[id * 4 + 3],
                            image.data[id * 4],
                            image.data[id * 4 + 1],
                            image.data[id * 4 + 2],
                        )
                    })
                    .collect();
            } else {
                return Err("Unsupported texture type");
            }
            Ok(Self {
                width: image.width,
                height: image.height,
                data,
                depth: image.depth,
            })
        } else {
            Err("Unsupported texture type")
        }
    }

    fn coords_to_index(coord: Vec2, height: usize) -> usize {
        coord.x as usize * height + coord.y as usize
    }

    pub fn argb_at_uv(&self, u: f32, v: f32) -> u32 {
        let u = u.abs() % 1.0;
        let v = v.abs() % 1.0;

        let uv = Vec2::new(u * self.width as f32, v * self.height as f32);
        let uv = uv.round();

        let id = Self::coords_to_index(uv, self.height);
        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb8(255, 255, 0, 255)
        }
    }
}
