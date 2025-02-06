use raylib::core::texture::Image;
use std::path::Path;
use std::path::PathBuf;
use structs::ComponentUpdateAction;
use structs::ComponentUpdateInstruction;

pub mod structs;

//const TEXTURE_PATH: &str = "../../textures";
const TEXTURE_PATH: &str = "textures";

fn load_image_from_file(file_path: PathBuf) -> Option<Image> {
    let image = Image::load_image(file_path.to_str().unwrap());
    return Some(image.unwrap());
}

pub fn get_available_textures(
    state: &structs::ComponentState,
) -> ComponentUpdateInstruction<structs::ImageFromFile> {
    let mut instruction: ComponentUpdateInstruction<structs::ImageFromFile> =
        ComponentUpdateInstruction::new();
    if !matches!(state.read_status, structs::TextureReadState::Untouched) {
        return instruction;
    }
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
            instruction.action = ComponentUpdateAction::Add;
            instruction.items = textures_by_file;
        }
    } else {
        println!("Could not read directory {:?}.", TEXTURE_PATH);
    }
    return instruction;
}
