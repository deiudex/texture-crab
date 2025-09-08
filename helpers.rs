use raylib::prelude::*;

pub fn get_centered_x_for_text(h: &RaylibDrawHandle, text: &str, font_size: i32, display_width: i32) -> i32 {
    //assumes output_size > 0
    let w: i32 = h.measure_text(text, font_size);
    return w * ((display_width / w) / 2);
}