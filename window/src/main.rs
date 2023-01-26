extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use shared::*;
use std::{path::Path, time::SystemTime};

pub mod reload;
use crate::reload::*;

static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

pub fn test_draw(x: u16, y: u16, color: u32) {
    unsafe {
        let index = x as usize * WIDTH + y as usize;
        if index < WIDTH * HEIGHT {
            BUFFER[x as usize * WIDTH + y as usize] = color;
        }
    }
}

fn main() {
    let mut shared_state = State {
        version: 1,
        time_passed: 0.0,
        draw_fn: test_draw,
        should_clear: true,
        clear_color: 0x00,
    };

    let mut app: Application;
    app = load_lib();

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if should_reload(last_modified) {
            println!("== NEW VERSION FOUND ==");
            app = reload(app);
            println!("== NEW VERSION LOADED ==");
            shared_state.version += 1;
            last_modified = SystemTime::now();
            app.setup(&shared_state);
        }

        // Clear screen if required
        if shared_state.should_clear {
            unsafe {
                for i in 0..BUFFER.len() {
                    BUFFER[i] = shared_state.clear_color;
                }
            }
        }

        shared_state.time_passed += 1.0;
        app.update(&shared_state);

        unsafe {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window.update_with_buffer(&BUFFER, WIDTH, HEIGHT).unwrap();
        }
    }
}
