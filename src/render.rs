use crate::components::structs::*;
use raylib::prelude::*;


pub fn render(dh: RaylibDrawHandle, component_state: &ComponentState) -> Option<ComponentUpdateInstruction<ComponentSpace>> {
    match component_state.comp_space {
        /*Once main is left, you can't really get back contextually.
        But all main 'func' should be invokable within edit.
        */
        ComponentSpace::Main => {
            /*
            1. Render a section to create a project.
            2. Render a section to load a project.
            3. Render a section to load recent proejcts.
            */
        },
        ComponentSpace::Edit => {
            //Main Project editor.
        },
        ComponentSpace::Settings => {

        },
        _ => {
            /* No one should get here */
            /* Reset to Main for now */
            let mut instruction: ComponentUpdateInstruction<ComponentSpace> = ComponentUpdateInstruction::new();
            instruction.action = ComponentUpdateAction::Change;
            Some(instruction.items.push(ComponentSpace::Main));
        }
    };
    return None;
}