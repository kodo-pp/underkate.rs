use super::move_trait::{HasMoveContext, MoveContext, Direction };
use super::multiwalk::{MultiWalk, MultiWalkState};
use super::walk::{Walk, WalkState};
use crate::graphics::texture::Texture;
use crate::resources;
use ggez::Context;
use super::direction_textured::DirectionTextured;

pub struct Player {
    front_texture: Texture,
    back_texture: Texture,
    leftward_texture: Texture,
    rightward_texture: Texture,
    move_context: MoveContext,
    walk_state: WalkState,
    multi_walk_state: MultiWalkState,
}

impl Player {
    pub fn new(ctx: &mut Context, move_context: MoveContext) -> Self {
        Self {
            front_texture: resources::static_texture(ctx, "player/front.png", 4.0),
            back_texture: resources::static_texture(ctx, "player/back.png", 4.0),
            leftward_texture: resources::static_texture(ctx, "player/leftward.png", 4.0),
            rightward_texture: resources::static_texture(ctx, "player/rightward.png", 4.0),
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

impl DirectionTextured for Player {
    fn texture_for_direction(&self, direction: Direction) -> &Texture {
        match direction {
            Direction::Backward => &self.front_texture,
            Direction::Forward => &self.back_texture,
            Direction::Left => &self.leftward_texture,
            Direction::Right => &self.rightward_texture,
        }
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
