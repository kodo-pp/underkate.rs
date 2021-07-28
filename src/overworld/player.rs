use super::direction_textured::DirectionTextured;
use super::hitbox::Hitbox;
use super::move_trait::{Direction, HasMoveContext, MoveContext};
use super::multiwalk::{MultiWalk, MultiWalkState};
use super::walk::{Walk, WalkState};
use crate::geometry::OverworldRect;
use crate::graphics::texture::Texture;
use crate::resources;
use ggez::Context;
use std::time::Duration;

const WALK_VELOCITY_ABS: f32 = 200.0;

struct AnimatableTexture {
    pub static_version: Texture,
    pub animated_version: Texture,
}

impl AnimatableTexture {
    pub fn new(static_version: Texture, animated_version: Texture) -> Self {
        Self { static_version, animated_version }
    }

    pub fn get(&self, is_animated: bool) -> &Texture {
        match is_animated {
            true => &self.animated_version,
            false => &self.static_version
        }
    }
}

pub struct Player {
    front_texture: AnimatableTexture,
    back_texture: AnimatableTexture,
    leftward_texture: AnimatableTexture,
    rightward_texture: AnimatableTexture,
    move_context: MoveContext,
    walk_state: WalkState,
    multi_walk_state: MultiWalkState,
}

impl Player {
    pub fn new(ctx: &mut Context, move_context: MoveContext) -> Self {
        Self {
            front_texture: AnimatableTexture::new(
                resources::static_texture(ctx, "player/front/0.png", 4.0),
                resources::animated_texture(
                    ctx,
                    &[
                        "player/front/0.png",
                        "player/front/1.png",
                        "player/front/2.png",
                        "player/front/3.png",
                        "player/front/4.png",
                        "player/front/5.png",
                        "player/front/6.png",
                        "player/front/7.png",
                    ],
                    Duration::from_secs_f64(0.1),
                    4.0
                ),
            ),
            back_texture: AnimatableTexture::new(
                resources::static_texture(ctx, "player/back/0.png", 4.0),
                resources::animated_texture(
                    ctx,
                    &[
                        "player/back/0.png",
                        "player/back/1.png",
                        "player/back/2.png",
                        "player/back/3.png",
                        "player/back/4.png",
                        "player/back/5.png",
                        "player/back/6.png",
                        "player/back/7.png",
                    ],
                    Duration::from_secs_f64(0.1),
                    4.0
                ),
            ),
            leftward_texture: AnimatableTexture::new(
                resources::static_texture(ctx, "player/leftward/0.png", 4.0),
                resources::animated_texture(
                    ctx,
                    &[
                        "player/leftward/0.png",
                        "player/leftward/1.png",
                        "player/leftward/2.png",
                        "player/leftward/3.png",
                        "player/leftward/4.png",
                        "player/leftward/5.png",
                    ],
                    Duration::from_secs_f64(0.1),
                    4.0
                ),
            ),
            rightward_texture: AnimatableTexture::new(
                resources::static_texture(ctx, "player/rightward/0.png", 4.0),
                resources::animated_texture(
                    ctx,
                    &[
                        "player/rightward/0.png",
                        "player/rightward/1.png",
                        "player/rightward/2.png",
                        "player/rightward/3.png",
                        "player/rightward/4.png",
                        "player/rightward/5.png",
                    ],
                    Duration::from_secs_f64(0.1),
                    4.0
                ),
            ),
            move_context,
            walk_state: WalkState::default(),
            multi_walk_state: MultiWalkState::new(WALK_VELOCITY_ABS),
        }
    }

    fn should_animate(&self) -> bool {
        !self.multi_walk_state.is_still()
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
            Direction::Backward => &self.front_texture.get(self.should_animate()),
            Direction::Forward => &self.back_texture.get(self.should_animate()),
            Direction::Left => &self.leftward_texture.get(self.should_animate()),
            Direction::Right => &self.rightward_texture.get(self.should_animate()),
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

impl Hitbox for Player {
    fn hitbox_at_origin(&self) -> OverworldRect<f32> {
        OverworldRect::new([-16.0, -28.0].into(), [16.0, 36.0].into())
    }
}
