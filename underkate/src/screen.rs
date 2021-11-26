use crate::default_runtime::DefaultRuntime;
use crate::game_context::GameContextRef;
use crate::resources::GlobalResourceStorage;
use crate::ui_event::UiEvent;
use ggez::GameResult;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub trait Screen {
    fn draw(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>) -> GameResult;
    fn update(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>) -> GameResult;
    fn handle_event(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>, event: UiEvent);
}
