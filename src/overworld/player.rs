use super::move_trait::{HasMoveContext, MoveContext};
use super::multiwalk::{MultiWalk, MultiWalkState};
use super::walk::{Walk, WalkState};
use crate::geometry::ScreenPoint;
use crate::graphics::texture::Texture;
use crate::graphics::Draw;
use crate::resources;
use ggez::{Context, GameResult};

pub struct Player {
    texture: Texture,
    move_context: MoveContext,
    walk_state: WalkState,
    multi_walk_state: MultiWalkState,
}

impl Player {
    pub fn new(ctx: &mut Context, move_context: MoveContext) -> Self {
        Self {
            texture: resources::static_texture(ctx, "player/player.png", 4.0),
            move_context,
            walk_state: WalkState::default(),
            multi_walk_state: MultiWalkState::default(),
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

impl Walk for Player {
    fn walk_state(&self) -> WalkState {
        self.walk_state
    }

    fn set_walk_state(&mut self, walk_state: WalkState) {
        self.walk_state = walk_state
    }
}

impl MultiWalk for Player {
    fn multi_walk_state(&self) -> MultiWalkState {
        self.multi_walk_state
    }

    fn set_multi_walk_state(&mut self, multi_walk_state: MultiWalkState) {
        self.multi_walk_state = multi_walk_state
    }
}
