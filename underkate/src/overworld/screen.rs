use super::room::{Room, CreationParams};
use crate::resources::{GlobalResourceStorage, ResourceStorageCloneExt};
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

pub struct OverworldScreen {
    room: Room,
}

impl OverworldScreen {
    pub fn new(global_resource_storage: &GlobalResourceStorage) -> OverworldScreen {
        let room_creation_params = global_resource_storage.get_cloned("_stub");

        OverworldScreen {
            room: Room::new(CreationParams::from_partial(room_creation_params, "_"), global_resource_storage),
        }
    }
}

impl Screen for OverworldScreen {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.room.draw(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.room.update(ctx)
    }

    fn handle_event(&mut self, ctx: &mut Context, event: UiEvent) {
        self.room.handle_event(ctx, event)
    }
}
