use super::geometry::TranslationContext;
use super::move_trait::{Direction, Move, MoveContext};
use super::player::Player;
use crate::graphics::Draw;
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::graphics::{self, Color};
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
}

impl Screen for OverworldScreen {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        draw_entity(ctx, &self.translation_context(), &mut self.player)?;
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn handle_event(&mut self, _ctx: &mut Context, event: UiEvent) {
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
