use raylib::prelude::*;

pub mod components;
pub mod shapes;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Crab Windows").build();
    let mut component_state = components::structs::ComponentState::init();
    let width: i32 = get_monitor_width(get_current_monitor()) / 2;
    let height: i32 = get_monitor_height(get_current_monitor()) / 2;
    rl.set_window_size(width, height);
    while !rl.window_should_close() {
        let instructions = components::get_available_textures(&component_state);
        component_state.update_textures(instructions);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Crab Windows", width / 2, 75, 20, Color::BLACK);
        shapes::render_main_screen_shapes(d, &component_state.textures);
    }
}
