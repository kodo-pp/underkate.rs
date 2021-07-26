use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

pub struct OverworldScreen;

impl Screen for OverworldScreen {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn handle_event(&mut self, _ctx: &mut Context, event: UiEvent) {
        eprintln!("Overworld: UI Event: {:?}", event);
    }
}
