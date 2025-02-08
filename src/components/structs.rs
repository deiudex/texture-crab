use raylib::texture::{Image, RenderTexture2D};
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
    pub canvases: Vec<RenderTexture2D>,
    pub read_status: TextureReadState,
    pub textures: Vec<ImageFromFile>,
}

impl ComponentState {
    pub fn init() -> ComponentState {
        return ComponentState {
            canvases: vec![],
            read_status: TextureReadState::Untouched,
            textures: vec![],
        };
    }
    pub fn update_textures(&mut self, i: ComponentUpdateInstruction<ImageFromFile>) {
        if matches!(i.action, ComponentUpdateAction::Add) {
            self.read_status = TextureReadState::Readable;
            self.textures = i.items;
        } else {
            self.read_status = TextureReadState::Empty;
        }
    }
    pub fn update_canvases(&mut self, i: ComponentUpdateInstruction<RenderTexture2D>) {
        match i.action {
            ComponentUpdateAction::Add => {
                for c in i.items {
                    self.canvases.push(c);
                }
            },
            ComponentUpdateAction::Remove => {
                for c in i.items {
                    self.canvases.swap_remove(self.canvases.iter().position(|v| *v == c).unwrap());
                }
            },
            _ => println!("An unknown update-component \"{:?}\" action was passed to the canvases... ignoring.", i.action),
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
    FAILURE,
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
