use crate::script::ScriptHandle;
use crate::game_context::GameContext;

pub async fn main(_script_handle: ScriptHandle, _context: GameContext) {
    println!("Hello world from script!");
}
