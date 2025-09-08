use crate::components::{spaces::{self, ScreenSpace}, structs::*};
use raylib::prelude::*;

pub mod home;

pub fn listen(
    dh: RaylibDrawHandle,
    component_state: &ComponentState,
) -> Option<ComponentUpdateInstruction<ComponentSpace>> {
    match component_state.comp_space {
        /*Once main is left, you can't really get back contextually.
        But all main 'func' should be invokable within edit.
        */
        ComponentSpace::Main => {
            
        }
        ComponentSpace::Edit => {
        }
        ComponentSpace::Settings => {}
        ComponentSpace::None => {
            /* No one should get here */
            /* Reset to Main for now */
            println!("ComponentSpace::None detected in render!");
            let mut instruction: ComponentUpdateInstruction<ComponentSpace> =
                ComponentUpdateInstruction::new();
            instruction.action = ComponentUpdateAction::Change;
            Some(instruction.items.push(ComponentSpace::Main));
        }
    };
    return None;
}

pub fn render(mut rh: RaylibHandle, t: &RaylibThread, state: ComponentState) {
} 