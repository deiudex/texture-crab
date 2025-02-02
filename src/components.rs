use raylib::prelude::*;
use std::ffi::CString;
use std::os::raw::*;
use std::path::Path;
use std::path::PathBuf;

pub mod structs;

const TEXTURE_PATH: &str = "../textures/";

unsafe fn get_file_path_list() -> ffi::FilePathList {
    let contact_point = CString::new(TEXTURE_PATH).unwrap().into_bytes_with_nul();
    let mut tmp: Vec<i8> = contact_point.into_iter().map(|c| c as i8).collect::<_>();
    let path_list = ffi::LoadDirectoryFiles(tmp.as_mut_ptr() as *const i8);
    return path_list;
}

fn convert_data(path: &str) -> *const c_char {
    let c_str = CString::new(path).unwrap();
    return c_str.as_ptr() as *const c_char;
}

unsafe fn load_image(file_path: PathBuf) -> Option<ffi::Image> {
    let image = ffi::LoadImage(convert_data(file_path.to_str().unwrap()));
    return Some(image);
}

unsafe fn get_available_textures() -> Vec<structs::ImageFromFile> {
    let texture_path_live: &Path = Path::new(TEXTURE_PATH);
    let mut textures_by_file = vec![];
    if texture_path_live.is_dir() {
        for entry in texture_path_live
            .read_dir()
            .expect("texture path marked as valid but was unreadable.")
        {
            if let Ok(entry) = entry {
                println!("found texture {:?}", entry.path());
                textures_by_file.push(structs::ImageFromFile {
                    name: String::from(entry.path().to_str().unwrap()),
                    image: load_image(entry.path()),
                });
            }
        }
    } else {
        textures_by_file.push(structs::ImageFromFile {
            name: String::from("_"),
            image: None,
        });
    }
    return textures_by_file;
}

pub fn test_texture_grabbing() {
    unsafe {
        get_available_textures();
    }
}

pub fn compose_overlay(rl: RaylibHandle) {}
