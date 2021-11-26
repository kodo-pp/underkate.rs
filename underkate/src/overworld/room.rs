use super::collide::Collide;
use super::geometry::TranslationContext;
use super::move_trait::{Direction, Move, MoveContext, Position};
use super::multiwalk::MultiWalk;
use super::pass_map::{BitmapPassMap, PassMap};
use super::passability_checker::{PassMapPassabilityChecker, PassabilityCheck};
use super::player::Player;
use super::walk::Walk;
use crate::game_context::GameContextRef;
use crate::graphics::texture::Texture;
use crate::graphics::Draw;
use crate::resources::{GlobalResourceStorage, ResourceStorageCloneExt};
use crate::script::rust_script::RustScript;
use crate::script::Script;
use crate::ui_event::UiEvent;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PartialCreationParams {
    pub background_path: String,
    pub pass_map_path: String,
    pub initial_player_states: HashMap<String, (Position, Direction)>,
    pub init_script: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct CreationParams {
    pub background_path: String,
    pub pass_map_path: String,
    pub player_position: Position,
    pub player_direction: Direction,
    pub init_script: Option<&'static str>,
}

impl CreationParams {
    pub fn from_partial(partial: PartialCreationParams, prev_room_name: &str) -> Self {
        let (position, direction) = partial.initial_player_states[prev_room_name];
        CreationParams {
            background_path: partial.background_path,
            pass_map_path: partial.pass_map_path,
            player_position: position,
            player_direction: direction,
            init_script: partial.init_script,
        }
    }
}

pub struct Room {
    background: Texture,
    pass_map: BitmapPassMap,
    player: Player,
    init_script: Option<Box<dyn Script>>,
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

        let init_script = params.init_script.map(|name| {
            let script: RustScript = global_resource_storage.get_cloned(name);
            Box::new(script) as Box<dyn Script>
        });

        Room {
            background: global_resource_storage.get_cloned(&params.background_path),
            pass_map: global_resource_storage.get_cloned(&params.pass_map_path),
            init_script,
            player,
        }
    }

    pub fn draw(&mut self, ggez: &mut Context, _ctx: GameContextRef<'_>) -> GameResult {
        self.background.draw(
            ggez,
            self.background.dimensions().to_vector().to_point() * 0.5,
        )?;
        draw_entity(ggez, &self.translation_context(), &mut self.player)?;
        Ok(())
    }

    pub fn update(&mut self, ggez: &mut Context, _ctx: GameContextRef<'_>) -> GameResult {
        let time_slice = ggez::timer::delta(ggez);

        let assumed_new_player_position = self.player.get_updated_position(time_slice);
        let maybe_new_player_position = self.player.find_passable_position(
            self.player.position(),
            assumed_new_player_position,
            &self.passability_checker(),
        );
        if let Some(new_player_position) = maybe_new_player_position {
            self.player.set_position(new_player_position)
        }

        Ok(())
    }

    pub fn handle_event(&mut self, _ggez: &mut Context, _ctx: GameContextRef<'_>, event: UiEvent) {
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

    fn passability_checker<'a>(&'a self) -> impl PassabilityCheck + 'a {
        PassMapPassabilityChecker::new(self.pass_map())
    }

    fn pass_map(&self) -> &impl PassMap {
        &self.pass_map
    }

    fn translation_context(&self) -> TranslationContext {
        TranslationContext
    }

    pub fn init_script_mut(&mut self) -> Option<&mut Box<dyn Script>> {
        self.init_script.as_mut()
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
