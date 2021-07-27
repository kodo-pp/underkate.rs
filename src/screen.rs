use crate::ui_event::UiEvent;
use ggez::{Context, GameResult};

pub trait Screen {
    fn draw(&mut self, ctx: &mut Context) -> GameResult;
    fn update(&mut self, ctx: &mut Context) -> GameResult;
    fn handle_event(&mut self, ctx: &mut Context, event: UiEvent);
}
