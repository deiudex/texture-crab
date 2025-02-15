use raylib::{color::Color, texture::{Image, RenderTexture2D}};
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
    pub fn init(x: f32, y: f32, w: f32, h: f32, t: Vec<String>) -> TextureBar {
        return TextureBar {
            main_bar: raylib::core::math::Rectangle{
             x: x,
             y: y,
             width: w,
             height: h,
        },
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

pub struct ComponentState {
    pub read_status: TextureReadState,
    pub textures: Vec<ImageFromFile>,
    pub comp_space: ComponentSpace,
}

impl ComponentState {
    pub fn init() -> ComponentState {
        return ComponentState {
            read_status: TextureReadState::Untouched,
            textures: vec![],
            comp_space: ComponentSpace::Main,
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
    pub fn update_space(&mut self, s: ComponentUpdateInstruction<ComponentSpace>) {
           if s.items.len() == 0 {
            println!("EMPTY SPACE CHANGE - RESETTING TO MAIN");
            self.comp_space = ComponentSpace::Main;
            return;
           }
           self.comp_space = s.items[0];
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
    Change,
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
