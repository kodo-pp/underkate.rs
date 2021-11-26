pub mod rust_script;

use crate::game_context::GameContext;
use crate::resources::GlobalResourceStorage;
use crate::screen::Screen;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;

mod tag {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct EventTag;
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct ScriptTag;

    pub type EventHandle = crate::handle::Handle<EventTag>;
    pub type EventHandleGenerator = crate::handle::HandleGenerator<EventTag>;
    pub type ScriptHandle = crate::handle::Handle<ScriptTag>;
    pub type ScriptHandleGenerator = crate::handle::HandleGenerator<ScriptTag>;
}
pub use tag::{EventHandle, EventHandleGenerator, ScriptHandle, ScriptHandleGenerator};

pub trait Runtime {
    fn subscribe(&mut self, event: EventHandle, script: ScriptHandle);
    fn raise_event(&mut self, event: EventHandle);
    fn wake_event(&self, script: ScriptHandle) -> Option<EventHandle>;
    fn start_script(&mut self, context: GameContext, script: &mut dyn Script);
}

pub trait Script {
    fn start(
        &mut self,
        script_handle: ScriptHandle,
        context: GameContext,
    ) -> Pin<Box<dyn Future<Output = ()>>>;
}
