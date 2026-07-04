use crate::components::structs::*;
use raylib::prelude::*;

pub mod components;
pub mod helpers;
pub mod input;
pub mod render;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .resizable()
        .title("Texture Crab")
        .build();
    let mut component_state = ComponentState::new();
    let width: i32 = get_monitor_width(get_current_monitor()) / 2;
    let height: i32 = get_monitor_height(get_current_monitor()) / 2;
    rl.set_window_size(width, height);
    if let Some(event) = components::get_available_textures(&component_state) {
        component_state.apply(event);
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        if let Some(event) = crate::render::home::render_home(&mut d, &component_state) {
            component_state.apply(event);
        }
    }
}

#[cfg(test)]
mod test {
    use raylib::prelude::*;

    #[test]
    fn test() {
        println!("Test 'build Raylib window'.");
        let (mut h, t) = test_window_init();
        println!("Test draw on window.");
        h.begin_drawing(&t);
    }

    fn test_window_init() -> (RaylibHandle, RaylibThread) {
        raylib::init()
            .size(1280, 720)
            .resizable()
            .title("Texture Crab")
            .build()
    }
}
