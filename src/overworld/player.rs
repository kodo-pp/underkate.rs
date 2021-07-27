use super::move_trait::{HasMoveContext, MoveContext};
use crate::geometry::ScreenPoint;
use crate::graphics::texture::Texture;
use crate::graphics::Draw;
use crate::resources;
use ggez::{Context, GameResult};

pub struct Player {
    texture: Texture,
    move_context: MoveContext,
}

impl Player {
    pub fn new(ctx: &mut Context, move_context: MoveContext) -> Self {
        Self {
            texture: resources::static_texture(ctx, "player/player.png", 4.0),
            move_context,
        }
    }
}

impl AsRef<MoveContext> for Player {
    fn as_ref(&self) -> &MoveContext {
        &self.move_context
    }
}

impl AsMut<MoveContext> for Player {
    fn as_mut(&mut self) -> &mut MoveContext {
        &mut self.move_context
    }
}

impl HasMoveContext for Player {}

impl Draw for Player {
    fn draw(&mut self, ctx: &mut Context, center_at: ScreenPoint<f32>) -> GameResult {
        self.texture.draw(ctx, center_at)
    }
}
