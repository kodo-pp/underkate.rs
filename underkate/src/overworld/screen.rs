use super::collide::Collide;
use super::geometry::TranslationContext;
use super::move_trait::{Direction, Move, MoveContext};
use super::multiwalk::MultiWalk;
use super::passability_checker::PassabilityChecker;
use super::player::Player;
use super::walk::Walk;
use crate::geometry::OverworldRect;
use crate::graphics::Draw;
use crate::resources::GlobalResourceStorage;
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

pub struct OverworldScreen {
    player: Player,
}

impl OverworldScreen {
    pub fn new(global_resource_storage: &GlobalResourceStorage) -> OverworldScreen {
        let player = Player::new(
            &global_resource_storage,
            MoveContext {
                position: [200.0, 200.0].into(),
                direction: Direction::Backward,
            },
        );

        OverworldScreen { player }
    }

    fn translation_context(&self) -> TranslationContext {
        TranslationContext
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

    fn passability_checker(&self) -> PassabilityChecker {
        PassabilityChecker::new(self.map_rect())
    }

    fn map_rect(&self) -> OverworldRect<f32> {
        OverworldRect::from_size([800.0, 600.0].into())
    }
}

impl Screen for OverworldScreen {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        draw_entity(ctx, &self.translation_context(), &mut self.player)?;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
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

    fn handle_event(&mut self, _ctx: &mut Context, event: UiEvent) {
        match event {
            UiEvent::KeyDown { key, .. } => {
                self.handle_key_down(key);
            }
            UiEvent::KeyUp { key, .. } => {
                self.handle_key_up(key);
            }
        }
        eprintln!("Overworld: UI Event: {:?}", event);
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
