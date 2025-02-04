use raylib::texture::Image;
use std::path::PathBuf;

pub struct ImageFromFile {
    pub name: String,
    pub image: Option<Image>,
}

pub struct OwnedPath {
    pub path: PathBuf,
}
