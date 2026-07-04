use crate::components::structs::*;
use raylib::prelude::*;

pub fn handle_main_input(h: &RaylibHandle) -> Option<AppEvent> {
    if h.is_key_pressed(consts::KeyboardKey::KEY_L) {
        return Some(AppEvent::OpenFileDialog);
    }
    None
}

pub fn delegate_inputs(h: &RaylibHandle, state: &ComponentState) -> Option<AppEvent> {
    match state.comp_space {
        ComponentSpace::Main => handle_main_input(h),
        ComponentSpace::Edit | ComponentSpace::Settings => None,
    }
}
