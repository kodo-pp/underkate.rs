use super::move_trait::{HasMoveContext, MoveContext};
use crate::geometry::ScreenPoint;
use crate::graphics::texture::Texture;
use crate::graphics::Draw;
use crate::resources;
use ggez::graphics::{FilterMode, Image};
use ggez::{Context, GameResult};

pub struct Player {
    texture: Texture,
    move_context: MoveContext,
}

impl Player {
    pub fn new(ctx: &mut Context, move_context: MoveContext) -> Self {
        let mut image = Image::from_bytes(ctx, resources::file("textures/player/player.png"))
            .expect("Unable to load player texture");
        image.set_filter(FilterMode::Nearest);

        let texture = Texture::new_static(image, 4.0);
        Self {
            texture,
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
