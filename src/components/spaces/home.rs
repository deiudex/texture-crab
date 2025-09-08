use crate::components::spaces::*;
use crate::render::home::*;
use raylib::prelude::*;

fn get_home_space() -> ScreenSpace {
    return ScreenSpace::init(
        get_home_space_keys(),
        get_home_space_elements()
    );
}

fn get_home_space_keys() -> KeyLayout {
    return KeyLayout::init(Vec::from([
        consts::KeyboardKey::KEY_L,
        consts::KeyboardKey::KEY_N,
        consts::KeyboardKey::KEY_Q,
    ]));
}

fn get_home_space_elements() -> Vec<ScreenElement> {
    return vec![];
}