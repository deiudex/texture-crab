use crate::components::structs;
use raylib::prelude::*;

fn render_texture_helper_btn(mut dh: RaylibDrawHandle, tcount: usize) {
    if tcount > 0 {
        dh.draw_rectangle(15, 15, 115, 25, Color::LIGHTSLATEGRAY);
        dh.draw_text("View Textures", 30, 20, 14, Color::WHITE);
    } else {
        dh.draw_rectangle(15, 15, 115, 25, Color::RED);
        dh.draw_text("NO TEXTURES", 18, 20, 15, Color::DARKVIOLET);
    }
}

pub fn render_main_screen_shapes(
    draw_handle: RaylibDrawHandle,
    textures: &Vec<structs::ImageFromFile>,
) {
    render_texture_helper_btn(draw_handle, textures.len());
}
