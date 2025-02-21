use raylib::prelude::*;

pub mod edit;
pub mod home;
pub mod settings;

pub struct ScreenSpace {
    pub layout: KeyLayout,
    pub elements: Vec<ScreenElement>,
}

pub struct KeyLayout {
    pub ray_keys: Vec<KeyboardKey>,
}

impl KeyLayout {
    pub fn init(v: Vec<KeyboardKey>) -> KeyLayout {
        return KeyLayout { ray_keys: vec![] };
    }
}

pub struct ScreenElement {
    pub name: String,
    pub render_method: fn(),
}

impl ScreenElement {
    pub fn init(n: String, f: fn()) -> ScreenElement {
        return ScreenElement {
            name: n,
            render_method: f,
        };
    }
}
