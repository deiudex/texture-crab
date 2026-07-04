use crate::components::structs::*;
use crate::helpers;
use raylib::prelude::*;

fn render_texture_helper(_dh: &mut RaylibDrawHandle, _images: &[ImageFromFile]) {}

fn render_stop_message(dh: &mut RaylibDrawHandle) {
    dh.draw_text(
        "No textures detected. Please create a textures folder with this executable and try again.",
        get_monitor_width(get_current_monitor()) / 4,
        get_monitor_height(get_current_monitor()) / 4,
        16,
        Color::RED,
    );
}

pub fn render_main_screen_shapes(dh: &mut RaylibDrawHandle, textures: &[ImageFromFile]) {
    let title_x = helpers::get_centered_x_for_text(
        dh,
        "Crab Windows",
        20,
        raylib::window::get_monitor_width(get_current_monitor()) / 2,
    );
    dh.draw_text("Crab Windows", title_x, 50, 20, Color::BLACK);
    if textures.is_empty() {
        render_stop_message(dh);
        return;
    }
    render_texture_helper(dh, textures);
}

pub fn render_home(dh: &mut RaylibDrawHandle, s: &ComponentState) -> Option<AppEvent> {
    render_main_screen_shapes(dh, &s.textures);
    None
}
