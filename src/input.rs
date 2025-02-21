use crate::components::structs::*;
use raylib::prelude::*;

pub fn handle_main_input(
    h: &RaylibHandle,
) -> Option<ComponentUpdateInstruction<ComponentUpdateAction>> {
    //need to pass the objects or their coordinates in order to do sufficient mouse-down logic.
    //for now, if key_pressed is 'L' for LOAD
    if h.is_key_pressed(consts::KeyboardKey::KEY_L) {
        let mut instruct: ComponentUpdateInstruction<ComponentUpdateAction> =
            ComponentUpdateInstruction::new();
        instruct.action = ComponentUpdateAction::DialogFile;
        return Some(instruct);
    }
    return None;
}

pub fn delegate_inputs(
    h: &RaylibHandle,
    state: &ComponentState,
) -> Option<ComponentUpdateInstruction<ComponentUpdateAction>> {
    //Just like render, baby!
    match state.comp_space {
        ComponentSpace::Main => {
            return handle_main_input(h);
        }
        ComponentSpace::Edit => {
            return None;
        }
        ComponentSpace::Settings => {
            return None;
        } //let render cry about "_"
        ComponentSpace::None => {
            return None;
        }
    }
}
