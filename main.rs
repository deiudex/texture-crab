use crate::components::spaces::*;
use crate::components::structs::*;
use raylib::prelude::*;

pub mod components;
pub mod helpers;
pub mod input;
pub mod render;
pub mod shapes;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .resizable()
        .title("Texture Crab")
        .build();
    let mut component_state = ComponentState::init();
    let width: i32 = get_monitor_width(get_current_monitor()) / 2;
    let height: i32 = get_monitor_height(get_current_monitor()) / 2;
    rl.set_window_size(width, height);
    let instructions = components::get_available_textures(&component_state);
    component_state.update_textures(instructions);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        //plug into home-render for now
        let instruct = crate::render::home::render_home(d, &component_state);
        if instruct.is_some() {
            component_state.update_space(instruct.unwrap());
        }

        //shapes::render_main_screen_shapes(d, &component_state.textures);
    }
}
