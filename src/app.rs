use ggez::{Context, ContextBuilder, GameResult, GameError};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::conf::WindowSetup;
use std::default::Default;

struct Underkate;

impl EventHandler<GameError> for Underkate {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        graphics::present(ctx)
    }
}

pub fn run() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Underkate", "kodopp")
        .window_setup(WindowSetup::default().title("Underkate"))
        .build()?;
    event::run(ctx, event_loop, Underkate);
}
