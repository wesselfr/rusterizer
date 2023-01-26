use glam::Vec2;
use shared::HEIGHT;
use shared::State;
use shared::WIDTH;

pub mod color;
use crate::color::*;

pub mod utils;
use crate::utils::*;

#[no_mangle]
pub fn setup(test: &State) {
    println!("Application version: {}", test.version);
    test.finalize();
}

#[no_mangle]
pub fn update(test: &mut State) {
    test.draw(0, 0, 0x00000000);

    let v0 = Vec2::new(0.0,0.0);
    let v1 = Vec2::new(500.0, 500.0);

    for i in 0..WIDTH*HEIGHT
    {
        let pos = index_to_coords(i, HEIGHT);
        let area = edge_function_cw(v0, v1, pos);

        if area > 0.0
        {
            test.draw(pos.x as u16, pos.y as u16, 0xffff30ff)
        }
        else
        {

        }
    }

    plotline(v0,v1, Color { a: 255, r: 255, g: 255, b: 255 }, &test);

    test.set_clear_color(0xff103030);
    //test.draw_text(0, 9, &format!("Current Version: {}", test.version));
    //test.draw_text(0, 10, "Custom draw function working.");
    //test.draw_text(0, 12, "Roguelike");
}
