use crate::overworld::screen::OverworldScreen;
use crate::resources::GlobalResourceStorage;
use crate::script::Runtime;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub struct GameContext {
    pub overworld_screen: Arc<Mutex<RefCell<OverworldScreen>>>,
    pub global_resource_storage: Arc<GlobalResourceStorage>,
    pub runtime: Arc<Mutex<RefCell<dyn Runtime>>>,
}

impl GameContext {
    pub fn as_context_ref(&self) -> GameContextRef<'_> {
        GameContextRef {
            overworld_screen: &self.overworld_screen,
            global_resource_storage: &self.global_resource_storage,
            runtime: &self.runtime,
        }
    }
}

pub struct GameContextRef<'a> {
    pub overworld_screen: &'a Arc<Mutex<RefCell<OverworldScreen>>>,
    pub global_resource_storage: &'a Arc<GlobalResourceStorage>,
    pub runtime: &'a Arc<Mutex<RefCell<dyn Runtime>>>,
}

impl GameContextRef<'_> {
    pub fn to_owned(&self) -> GameContext {
        GameContext {
            overworld_screen: Arc::clone(self.overworld_screen),
            global_resource_storage: Arc::clone(self.global_resource_storage),
            runtime: Arc::clone(self.runtime),
        }
    }
}
