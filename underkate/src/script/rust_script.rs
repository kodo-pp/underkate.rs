use super::{Script, ScriptHandle};
use crate::game_context::GameContext;
use std::future::Future;
use std::pin::Pin;

type StartFn = fn(ScriptHandle, GameContext) -> Pin<Box<dyn Future<Output = ()>>>;

#[derive(Clone, Copy)]
pub struct RustScript {
    start_fn: StartFn,
}

impl RustScript {
    pub fn new(start_fn: StartFn) -> Self {
        Self { start_fn }
    }
}

impl Script for RustScript {
    fn start(
        self: &mut Self,
        script_handle: ScriptHandle,
        context: GameContext,
    ) -> Pin<Box<dyn Future<Output = ()>>> {
        (self.start_fn)(script_handle, context)
    }
}
