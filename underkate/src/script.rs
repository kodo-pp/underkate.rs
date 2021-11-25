use crate::resources::GlobalResourceStorage;
use crate::screen::Screen;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

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
}

pub struct ScriptContext {
    screen: Arc<Mutex<RefCell<dyn Screen>>>,
    global_resource_storage: Arc<GlobalResourceStorage>,
    runtime: Arc<Mutex<RefCell<dyn Runtime>>>,
}

pub trait Script {
    fn start(
        self: Pin<&mut Self>,
        script_handle: ScriptHandle,
        context: ScriptContext,
    ) -> Pin<Box<dyn Future<Output = ()>>>;
}
