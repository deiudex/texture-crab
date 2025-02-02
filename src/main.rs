use raylib::prelude::*;

pub mod components;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Crab Windows").build();
    let width: i32 = get_monitor_width(get_current_monitor());
    let height: i32 = get_monitor_height(get_current_monitor());
    rl.set_window_size(width / 2, height / 2);
    //components::test_texture_grabbing()
    while !rl.window_should_close() {
        //components::compose_overlay(rl);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", width / 4, height / 4, 20, Color::BLACK);
    }
}
