use super::move_trait::{Direction, Move};
use crate::geometry::ScreenPoint;
use crate::graphics::texture::Texture;
use crate::graphics::Draw;
use ggez::{Context, GameResult};

pub trait DirectionTextured: Move {
    fn texture_for_direction(&self, direction: Direction) -> &Texture;
}

impl<T: DirectionTextured> Draw for T {
    fn draw(&self, ctx: &mut Context, center_at: ScreenPoint<f32>) -> GameResult {
        self.texture_for_direction(self.direction())
            .draw(ctx, center_at)
    }
}
