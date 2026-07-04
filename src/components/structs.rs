use egui_file_dialog::FileDialog;
use raylib::{color::Color, texture::Image};
use std::path::PathBuf;

pub struct ImageFromFile {
    pub name: String,
    pub image: Option<Image>,
}

pub struct TextureBar {
    pub main_bar: raylib::core::math::Rectangle,
    pub color: Color,
    pub texture_labels: Vec<String>,
}

impl TextureBar {
    pub fn new(x: f32, y: f32, w: f32, h: f32, t: Vec<String>) -> TextureBar {
        TextureBar {
            main_bar: raylib::core::math::Rectangle { x, y, width: w, height: h },
            color: Color::DARKGOLDENROD,
            texture_labels: t,
        }
    }
}

pub struct OwnedPath {
    pub path: PathBuf,
}

pub struct IntRectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub enum ComponentSpace {
    Main,
    Edit,
    Settings,
}

pub enum TextureReadState {
    Readable,
    Problematic,
    Empty,
    Untouched,
}

pub enum AppEvent {
    TexturesLoaded(Vec<ImageFromFile>),
    TexturesEmpty,
    SpaceChanged(ComponentSpace),
    OpenFileDialog,
}

pub struct ComponentState {
    pub read_status: TextureReadState,
    pub textures: Vec<ImageFromFile>,
    pub comp_space: ComponentSpace,
    pub file_dialog: FileDialog,
}

impl ComponentState {
    pub fn new() -> ComponentState {
        ComponentState {
            read_status: TextureReadState::Untouched,
            textures: vec![],
            comp_space: ComponentSpace::Main,
            file_dialog: FileDialog::new(),
        }
    }

    pub fn apply(&mut self, event: AppEvent) {
        match event {
            AppEvent::TexturesLoaded(textures) => {
                self.read_status = TextureReadState::Readable;
                self.textures = textures;
            }
            AppEvent::TexturesEmpty => {
                self.read_status = TextureReadState::Empty;
            }
            AppEvent::SpaceChanged(space) => {
                self.comp_space = space;
            }
            AppEvent::OpenFileDialog => {}
        }
    }
}
