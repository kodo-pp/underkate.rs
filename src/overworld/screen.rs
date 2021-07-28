use super::geometry::TranslationContext;
use super::move_trait::{Direction, Move, MoveContext};
use super::multiwalk::MultiWalk;
use super::player::Player;
use super::walk::Walk;
use crate::graphics::Draw;
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

pub struct OverworldScreen {
    player: Player,
}

impl OverworldScreen {
    pub fn new(ctx: &mut Context) -> OverworldScreen {
        let player = Player::new(
            ctx,
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
        const WALK_VELOCITY_ABS: f32 = 100.0;

        match key {
            KeyCode::Up => {
                self.player.start_walking_forward(WALK_VELOCITY_ABS);
            }
            KeyCode::Down => {
                self.player.start_walking_backward(WALK_VELOCITY_ABS);
            }
            KeyCode::Left => {
                self.player.start_walking_left(WALK_VELOCITY_ABS);
            }
            KeyCode::Right => {
                self.player.start_walking_right(WALK_VELOCITY_ABS);
            }
            _ => (),
        }
    }

    fn handle_key_up(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                self.player.stop_walking_forward();
            }
            KeyCode::Down => {
                self.player.stop_walking_backward();
            }
            KeyCode::Left => {
                self.player.stop_walking_left();
            }
            KeyCode::Right => {
                self.player.stop_walking_right();
            }
            _ => (),
        }
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
        self.player.update_position(time_slice);
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
