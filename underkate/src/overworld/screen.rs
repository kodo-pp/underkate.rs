use super::room::Room;
use crate::game_context::GameContextRef;
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::graphics::{self, Color};
use ggez::GameResult;

pub struct OverworldScreen {
    room: Option<Room>,
}

impl OverworldScreen {
    pub fn new() -> OverworldScreen {
        OverworldScreen { room: None }
    }

    pub fn load_room(&mut self, ctx: GameContextRef<'_>, room: Room) {
        self.room = Some(room);
        self.init_room(ctx);
    }

    fn init_room(&mut self, ctx: GameContextRef<'_>) {
        if let Some(script) = self.room.as_mut().unwrap().init_script_mut() {
            ctx.runtime
                .lock()
                .unwrap()
                .borrow_mut()
                .start_script(ctx.to_owned(), script.as_mut());
        }
    }
}

impl Screen for OverworldScreen {
    fn draw(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>) -> GameResult {
        graphics::clear(ggez, Color::BLACK);
        self.room.as_mut().unwrap().draw(ggez, ctx)
    }

    fn update(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>) -> GameResult {
        self.room.as_mut().unwrap().update(ggez, ctx)
    }

    fn handle_event(&mut self, ggez: &mut ggez::Context, ctx: GameContextRef<'_>, event: UiEvent) {
        self.room.as_mut().unwrap().handle_event(ggez, ctx, event)
    }
}
