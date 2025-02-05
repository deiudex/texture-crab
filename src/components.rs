use raylib::core::texture::Image;
use std::path::Path;
use std::path::PathBuf;

pub mod structs;

//const TEXTURE_PATH: &str = "../../textures";
const TEXTURE_PATH: &str = "textures";

fn load_image_from_file(file_path: PathBuf) -> Option<Image> {
    let image = Image::load_image(file_path.to_str().unwrap());
    return Some(image.unwrap());
}

pub fn get_available_textures() -> Vec<structs::ImageFromFile> {
    let texture_path_live: &Path = Path::new(TEXTURE_PATH);
    let mut textures_by_file = vec![];
    if texture_path_live.is_dir() {
        if let Ok(entries) = std::fs::read_dir(texture_path_live) {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("found texture {:?}", entry.path());
                    textures_by_file.push(structs::ImageFromFile {
                        name: String::from(entry.path().to_str().unwrap()),
                        image: load_image_from_file(entry.path()),
                    });
                }
            }
        }
    } else {
        println!("Could not read directory {:?}.", TEXTURE_PATH);
    }

    return textures_by_file;
}
