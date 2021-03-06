use super::hitbox::Hitbox;
use super::move_trait::{Direction, HasMoveContext, MoveContext};
use super::multiside::MoveAnimatedMultiside;
use super::multiwalk::{MultiWalk, MultiWalkState};
use super::walk::{Walk, WalkState};
use crate::geometry::OverworldRect;
use crate::graphics::texture::Texture;
use crate::resources::{GlobalResourceStorage, ResourceStorageCloneExt};

const WALK_VELOCITY_ABS: f32 = 200.0;

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
    pub fn new(resources: &GlobalResourceStorage, move_context: MoveContext) -> Self {
        Self {
            front_texture: resources.get_cloned("overworld/player/front"),
            back_texture: resources.get_cloned("overworld/player/back"),
            leftward_texture: resources.get_cloned("overworld/player/leftward"),
            rightward_texture: resources.get_cloned("overworld/player/rightward"),
            move_context,
            walk_state: WalkState::default(),
            multi_walk_state: MultiWalkState::new(WALK_VELOCITY_ABS),
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

impl MoveAnimatedMultiside for Player {
    fn texture_for_direction(&self, direction: Direction) -> &Texture {
        match direction {
            Direction::Backward => &self.front_texture,
            Direction::Forward => &self.back_texture,
            Direction::Left => &self.leftward_texture,
            Direction::Right => &self.rightward_texture,
        }
    }

    fn is_moving(&self) -> bool {
        !self.multi_walk_state.is_still()
    }

    fn direction(&self) -> Direction {
        self.move_context.direction
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

impl Hitbox for Player {
    fn hitbox_at_origin(&self) -> OverworldRect<f32> {
        OverworldRect::new([-16.0, -28.0].into(), [16.0, 36.0].into())
    }
}
