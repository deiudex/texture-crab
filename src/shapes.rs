use crate::components::structs;
use raylib::prelude::*;

fn render_texture_helper_btn(mut dh: RaylibDrawHandle, tcount: usize) {
    let mut helper_color = Color::LIGHTSLATEGRAY;
    let mut helper_text = "View Textures";
    let mut text_color = Color::WHITE;
    if tcount == 0 {
        helper_color = Color::RED;
        helper_text = "NO TEXTURES";
        text_color = Color::DARKVIOLET;
    }
    dh.draw_rectangle(15, 15, 115, 25, helper_color);
    dh.draw_text(helper_text, 18, 20, 15, text_color);
}

pub fn render_main_screen_shapes(
    draw_handle: RaylibDrawHandle,
    textures: &Vec<structs::ImageFromFile>,
) {
    render_texture_helper_btn(draw_handle, textures.len());
}
