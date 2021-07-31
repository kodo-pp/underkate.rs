use super::move_trait::Direction;
use crate::geometry::ScreenPoint;
use crate::graphics::Draw;
use crate::graphics::texture::Texture;
use ggez::{Context, GameResult};

pub trait MoveAnimatedMultiside {
    fn texture_for_direction(&self, direction: Direction) -> &Texture;
    fn is_moving(&self) -> bool;
    fn direction(&self) -> Direction;
}

impl<T: MoveAnimatedMultiside> Draw for T {
    fn draw(&self, ctx: &mut Context, center_at: ScreenPoint<f32>) -> GameResult {
        let texture = self.texture_for_direction(self.direction());
        if self.is_moving() {
            texture.draw(ctx, center_at)
        } else {
            texture.static_draw(ctx, center_at)
        }
    }
}
