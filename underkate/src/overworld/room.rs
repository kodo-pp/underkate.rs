use super::collide::Collide;
use super::geometry::TranslationContext;
use super::move_trait::{Direction, Move, MoveContext, Position};
use super::multiwalk::MultiWalk;
use super::passability_checker::PassabilityChecker;
use super::player::Player;
use super::walk::Walk;
use crate::geometry::OverworldRect;
use crate::graphics::texture::Texture;
use crate::graphics::Draw;
use crate::resources::{GlobalResourceStorage, ResourceStorage};
use crate::ui_event::UiEvent;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

#[derive(Debug)]
pub struct CreationParams {
    pub background_path: String,
    pub pass_map_path: String,
    pub player_position: Position,
    pub player_direction: Direction,
}

pub struct Room {
    background: Texture,
    player: Player,
}

impl Room {
    pub fn new(params: CreationParams, global_resource_storage: &GlobalResourceStorage) -> Room {
        let player = Player::new(
            &global_resource_storage,
            MoveContext {
                position: params.player_position,
                direction: params.player_direction,
            },
        );

        Room {
            background: global_resource_storage.get(&params.background_path).clone(),
            player,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.background.draw(ctx, self.background.dimensions().to_vector().to_point() * 0.5)?;
        draw_entity(ctx, &self.translation_context(), &mut self.player)?;
        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let time_slice = ggez::timer::delta(ctx);

        let assumed_new_player_position = self.player.get_updated_position(time_slice);
        if let Some(new_player_position) = self.player.find_passable_position(
            self.player.position(),
            assumed_new_player_position,
            &self.passability_checker(),
        ) {
            self.player.set_position(new_player_position)
        }

        Ok(())
    }

    pub fn handle_event(&mut self, _ctx: &mut Context, event: UiEvent) {
        match event {
            UiEvent::KeyDown { key, .. } => {
                self.handle_key_down(key);
            }
            UiEvent::KeyUp { key, .. } => {
                self.handle_key_up(key);
            }
        }
    }

    fn update_player_direction(&mut self) {
        let multi_walk_state = self.player.multi_walk_state();
        if multi_walk_state.is_still() {
            return;
        }

        let [x, y]: [f32; 2] = multi_walk_state.resulting_velocity().into();

        let direction = if x.abs() >= y.abs() {
            if x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if y > 0.0 {
                Direction::Backward
            } else {
                Direction::Forward
            }
        };

        self.player.set_direction(direction);
    }

    fn handle_key_down(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                self.player.start_walking_forward();
                self.update_player_direction();
            }
            KeyCode::Down => {
                self.player.start_walking_backward();
                self.update_player_direction();
            }
            KeyCode::Left => {
                self.player.start_walking_left();
                self.update_player_direction();
            }
            KeyCode::Right => {
                self.player.start_walking_right();
                self.update_player_direction();
            }
            _ => (),
        }
    }

    fn handle_key_up(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                self.player.stop_walking_forward();
                self.update_player_direction();
            }
            KeyCode::Down => {
                self.player.stop_walking_backward();
                self.update_player_direction();
            }
            KeyCode::Left => {
                self.player.stop_walking_left();
                self.update_player_direction();
            }
            KeyCode::Right => {
                self.player.stop_walking_right();
                self.update_player_direction();
            }
            _ => (),
        }
    }

    fn passability_checker(&self) -> PassabilityChecker {
        PassabilityChecker::new(self.map_rect())
    }

    fn map_rect(&self) -> OverworldRect<f32> {
        OverworldRect::from_size([800.0, 600.0].into())
    }

    fn translation_context(&self) -> TranslationContext {
        TranslationContext
    }
}

fn draw_entity(
    ctx: &mut Context,
    translation_context: &TranslationContext,
    entity: &mut (impl Draw + Move),
) -> GameResult {
    let screen_position = translation_context.to_screen(entity.position());
    entity.draw(ctx, screen_position)
}
