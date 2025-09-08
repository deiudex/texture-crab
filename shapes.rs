use crate::components::structs;
use crate::helpers;
use raylib::prelude::*;

fn render_texture_helper(mut dh: RaylibDrawHandle, images: &Vec<structs::ImageFromFile>) {
    let scroller_texture = ffi::Texture2D {
        id: 101,
        width: 100,
        height: get_monitor_height(get_current_monitor()) / 2,
        mipmaps: 2,
        format: 1,
    };
    let scroller_depth_texture = ffi::Texture2D {
        id: 102,
        width: 100,
        height: get_monitor_height(get_current_monitor()) / 2,
        mipmaps: 2,
        format: 1,
    };
}

fn render_stop_message(mut dh: RaylibDrawHandle) {
    dh.draw_text(
        "No textures detected. Please create a textures folder with this executable and try again.",
        get_monitor_width(get_current_monitor()) / 4,
        get_monitor_height(get_current_monitor()) / 4,
        16,
        Color::RED,
    );
}

pub fn render_main_screen_shapes(
    mut draw_handle: RaylibDrawHandle,
    textures: &Vec<structs::ImageFromFile>,
) {
    draw_handle.draw_text(
        "Crab Windows",
        helpers::get_centered_x_for_text(
            &draw_handle,
            "Crab Windows",
            20,
            raylib::window::get_monitor_width(get_current_monitor()) / 2,
        ),
        50,
        20,
        Color::BLACK,
    );
    if textures.len() == 0 {
        render_stop_message(draw_handle);
        return;
    }
    render_texture_helper(draw_handle, textures);
}

