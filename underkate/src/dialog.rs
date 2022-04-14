use crate::script::{Script, ScriptHandle};
use crate::game_context::GameContext;
use std::future::Future;
use std::pin::Pin;

pub struct Dialog {
    frames: Vec<DialogFrame>,
}

struct DialogFrame {
    text: String,
}

impl Script for Dialog {
    fn start(
        &mut self,
        script_handle: ScriptHandle,
        context: GameContext,
    ) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async {
            let runtime_lock = context.runtime.lock().unwrap();
            let runtime_ref = runtime_lock.borrow();
            runtime_ref.run_with_ggez()
        })
    }
}
