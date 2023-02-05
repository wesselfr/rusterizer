extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use shared::{camera::Camera, texture::Texture, transform::Transform, *};
use std::{
    path::Path,
    time::{Instant, SystemTime},
};

pub mod reload;
use crate::reload::*;

static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

pub fn test_draw(x: u16, y: u16, color: u32) {
    unsafe {
        let index = x as usize * WIDTH + y as usize;
        if index < WIDTH * HEIGHT {
            BUFFER[index] = color;
        }
    }
}

fn main() {
    let mut shared_state = State {
        version: 1,
        time_passed: 0.0,
        draw_fn: test_draw,
        meshes: Vec::new(),
        textures: Vec::new(),
        camera: Camera {
            fov: WIDTH as f32 / HEIGHT as f32,
            transform: Transform::from_translation(glam::vec3(0.0, 1.5, 6.0)),
            ..Default::default()
        },
        should_clear: true,
        clear_color: 0x00,
    };

    let mut app: Application;
    app = load_lib();
    app.setup(&shared_state);

    let mut last_modified = SystemTime::now();

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
    //window.limit_update_rate(None);

    let mut dt = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = Instant::now();
        if should_reload(last_modified) {
            println!("== NEW VERSION FOUND ==");
            app = reload(app);
            println!("== NEW VERSION LOADED ==");
            shared_state.version += 1;
            last_modified = SystemTime::now();
            app.setup(&shared_state);
        }

        // Handle input
        window
            .get_keys_pressed(minifb::KeyRepeat::Yes)
            .iter()
            .for_each(|key| match key {
                Key::W => {
                    shared_state
                        .camera
                        .transform
                        .translate(shared_state.camera.transform.forward() * 100.0 * dt);
                }
                Key::S => {
                    shared_state
                        .camera
                        .transform
                        .translate(-shared_state.camera.transform.forward() * 100.0 * dt);
                }
                Key::A => {
                    shared_state
                        .camera
                        .transform
                        .translate(shared_state.camera.transform.up() * 100.0 * dt);
                }
                Key::D => {
                    shared_state
                        .camera
                        .transform
                        .translate(-shared_state.camera.transform.up() * 100.0 * dt);
                }
                _ => (),
            });
        // Clear screen if required
        if shared_state.should_clear {
            unsafe {
                for i in 0..BUFFER.len() {
                    BUFFER[i] = shared_state.clear_color;
                }
            }
        }

        app.update(&shared_state);
        let elapsed_time = start_time.elapsed();
        println!("Frame render time: {} ms", elapsed_time.as_millis());

        unsafe {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window.update_with_buffer(&BUFFER, WIDTH, HEIGHT).unwrap();
        }

        let elapsed_time = start_time.elapsed();
        println!(
            "Total time: {} ms ({} FPS)",
            elapsed_time.as_millis(),
            1000 / elapsed_time.as_millis().max(1)
        );
        dt = elapsed_time.as_secs_f32();
        shared_state.time_passed += elapsed_time.as_secs_f32();
    }
}
