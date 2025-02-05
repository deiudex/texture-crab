use raylib::prelude::*;

pub mod components;
pub mod shapes;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Crab Windows").build();
    let width: i32 = get_monitor_width(get_current_monitor());
    let height: i32 = get_monitor_height(get_current_monitor());
    let mut assessable_textures: Vec<components::structs::ImageFromFile> =
        components::get_available_textures();
    rl.set_window_size(width / 2, height / 2);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", width / 4, height / 4, 20, Color::BLACK);
        shapes::render_main_screen_shapes(d, &assessable_textures);
    }
}
