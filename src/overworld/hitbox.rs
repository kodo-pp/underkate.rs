use super::move_trait::Position;
use crate::geometry::{OverworldDimensions, OverworldRect};

pub trait Hitbox {
    fn hitbox_at_origin(&self) -> OverworldRect<f32>;

    fn hitbox_at(&self, point: Position) -> OverworldRect<f32> {
        self.hitbox_at_origin().translate(point.to_vector())
    }
}

pub trait CenteredHitbox {
    fn hitbox_dimensions(&self) -> OverworldDimensions<f32>;
}

impl<T: CenteredHitbox> Hitbox for T {
    fn hitbox_at_origin(&self) -> OverworldRect<f32> {
        let dimensions = self.hitbox_dimensions();
        OverworldRect::from_size(dimensions)
            .translate([-dimensions.width / 2.0, -dimensions.height / 2.0].into())
    }
}
