pub mod bitmap_pass_map;

pub use bitmap_pass_map::BitmapPassMap;
use crate::geometry::OverworldRect;

pub trait PassMap {
    fn collides_with(&self, rect: &OverworldRect<f32>) -> bool;
    fn bounds(&self) -> OverworldRect<f32>;
}
