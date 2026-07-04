use raylib::core::texture::Image;
use std::path::{Path, PathBuf};
use structs::{AppEvent, ImageFromFile, TextureReadState};

pub mod spaces;
pub mod structs;

const TEXTURE_PATH: &str = "textures";

fn load_image_from_file(file_path: PathBuf) -> Option<Image> {
    Image::load_image(file_path.to_str().unwrap()).ok()
}

pub fn get_available_textures(state: &structs::ComponentState) -> Option<AppEvent> {
    if !matches!(state.read_status, TextureReadState::Untouched) {
        return None;
    }
    let texture_path = Path::new(TEXTURE_PATH);
    if !texture_path.is_dir() {
        println!("Could not read directory {:?}.", TEXTURE_PATH);
        return Some(AppEvent::TexturesEmpty);
    }
    let textures: Vec<ImageFromFile> = std::fs::read_dir(texture_path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            println!("found texture {:?}", entry.path());
            Some(ImageFromFile {
                name: entry.path().to_str().unwrap().to_string(),
                image: load_image_from_file(entry.path()),
            })
        })
        .collect();

    if textures.is_empty() {
        Some(AppEvent::TexturesEmpty)
    } else {
        Some(AppEvent::TexturesLoaded(textures))
    }
}
