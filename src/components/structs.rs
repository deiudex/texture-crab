use raylib::texture::Image;
use std::path::PathBuf;

pub struct ImageFromFile {
    pub name: String,
    pub image: Option<Image>,
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

pub struct ComponentState {
    pub read_status: TextureReadState,
    pub textures: Vec<ImageFromFile>,
}

impl ComponentState {
    pub fn init() -> ComponentState {
        return ComponentState {
            read_status: TextureReadState::Untouched,
            textures: vec![],
        };
    }
    fn update_read_status(&mut self, s: TextureReadState) {
        self.read_status = s;
    }
    pub fn add_textures(&mut self, textures: Vec<ImageFromFile>) {
        self.textures = textures;
    }
    pub fn update(&mut self, i: ComponentUpdateInstruction<ImageFromFile>) {
        if matches!(i.action, ComponentUpdateAction::Add) {
            self.update_read_status(TextureReadState::Readable);
            self.add_textures(i.items);
        }
    }
}

pub enum TextureReadState {
    Readable,
    Problematic,
    Empty,
    Untouched,
}

pub enum ComponentUpdateAction {
    Blank,
    Create,
    Add,
    Remove,
    Delete,
}

pub struct ComponentUpdateInstruction<T> {
    pub action: ComponentUpdateAction,
    pub items: Vec<T>,
}

impl<T> ComponentUpdateInstruction<T> {
    pub fn new() -> ComponentUpdateInstruction<T> {
        Self {
            action: ComponentUpdateAction::Blank,
            items: Vec::new(),
        }
    }
}
