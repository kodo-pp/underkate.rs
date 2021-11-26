use crate::game_context::GameContextRef;
use crate::ui_event::UiEvent;
use ggez::GameResult;

pub trait Screen {
    fn draw(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>) -> GameResult;
    fn update(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>) -> GameResult;
    fn handle_event(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>, event: UiEvent);
}
