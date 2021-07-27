use crate::geometry::{OverworldPoint, ScreenPoint};

#[derive(Debug, Copy, Clone)]
pub struct TranslationContext;

impl TranslationContext {
    pub fn to_screen<T: Copy>(&self, point: OverworldPoint<T>) -> ScreenPoint<T> {
        point.cast_unit()
    }
}
