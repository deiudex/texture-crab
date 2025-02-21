use crate::components::spaces::*;
use crate::render::home::*;
use raylib::prelude::*;

fn get_home_space() -> ScreenSpace {}

fn get_home_space_keys() -> KeyLayout {
    return KeyLayout::new(Vec::from([
        consts::KeyboardKey::KEY_L,
        consts::KeyboardKey::KEY_N,
        consts::KeyboardKey::KEY_Q,
    ]));
}
