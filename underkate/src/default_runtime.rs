use crate::game_context::GameContext;
use crate::script::{EventHandle, Runtime, Script, ScriptHandle, ScriptHandleGenerator};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, RawWaker, RawWakerVTable, Waker};

#[derive(Debug, Default)]
struct SubscriberList {
    pub once: Vec<ScriptHandle>,
}

struct ScriptState {
    wake_event: Option<EventHandle>,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl ScriptState {
    fn from_future(future: Pin<Box<dyn Future<Output = ()>>>) -> Self {
        Self {
            wake_event: None,
            future,
        }
    }
}

const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

fn make_noop_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE)
}

fn make_noop_waker() -> Waker {
    // SAFETY: all unsafe operations are no-ops (except clone, which just creates a new no-op waker).
    unsafe { Waker::from_raw(make_noop_raw_waker()) }
}

fn noop_clone(_: *const ()) -> RawWaker {
    make_noop_raw_waker()
}

fn noop(_: *const ()) {}

pub struct DefaultRuntime {
    subscribers: HashMap<EventHandle, SubscriberList>,
    scripts: HashMap<ScriptHandle, ScriptState>,
    script_handle_generator: ScriptHandleGenerator,
}

impl DefaultRuntime {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            scripts: HashMap::new(),
            script_handle_generator: ScriptHandleGenerator::new(),
        }
    }

    fn resume_script(&mut self, script: ScriptHandle, wake_event: Option<EventHandle>) {
        let waker = make_noop_waker();
        let mut context = Context::from_waker(&waker);
        let poll = {
            let script_state = self.scripts.get_mut(&script).unwrap();
            script_state.wake_event = wake_event;
            script_state.future.as_mut().poll(&mut context)
        };

        // Remove the script if it has finished running.
        if poll.is_ready() {
            // XXX: maybe we shouldn't really do this.
            for (_event, list) in self.subscribers.iter_mut() {
                list.once.retain(|&x| x != script);
            }
            self.scripts.remove(&script);
        }
    }
}

impl Runtime for DefaultRuntime {
    fn subscribe(&mut self, event: EventHandle, script: ScriptHandle) {
        self.subscribers.entry(event).or_default().once.push(script)
    }

    fn raise_event(&mut self, event: EventHandle) {
        if let Some(list) = self.subscribers.remove(&event) {
            for script in list.once {
                self.resume_script(script, Some(event));
            }
        }
    }

    fn wake_event(&self, script: ScriptHandle) -> Option<EventHandle> {
        self.scripts[&script].wake_event
    }

    fn start_script(&mut self, context: GameContext, script: &mut dyn Script) {
        let handle = self.script_handle_generator.gen_handle();
        let future = script.start(handle, context);
        let state = ScriptState::from_future(future);
        self.scripts.insert(handle, state);
        self.resume_script(handle, None);
    }
}
