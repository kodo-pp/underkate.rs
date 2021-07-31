use super::move_trait::Position;
use crate::geometry::OverworldRect;

pub trait Hitbox {
    fn hitbox_at_origin(&self) -> OverworldRect<f32>;

    fn hitbox_at(&self, point: Position) -> OverworldRect<f32> {
        self.hitbox_at_origin().translate(point.to_vector())
    }
}
