#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResourceType {
    Texture,
    Room,
    PassMap,
    RustScript,
}

impl ResourceType {
    pub fn rust_type(&self) -> &'static str {
        match self {
            Self::Texture => "crate::graphics::texture::Texture",
            Self::Room => "crate::overworld::room::PartialCreationParams",
            Self::RustScript => "crate::script::rust_script::RustScript",
            Self::PassMap => "crate::overworld::pass_map::BitmapPassMap",
        }
    }
}
