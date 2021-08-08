use super::room::{CreationParams, Room};
use super::move_trait::Direction;
use crate::resources::GlobalResourceStorage;
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

pub struct OverworldScreen {
    room: Room,
}

impl OverworldScreen {
    pub fn new(global_resource_storage: &GlobalResourceStorage) -> OverworldScreen {
        let room_creation_params = CreationParams {
            background_path: String::from("overworld/rooms/_stub/bg"),
            pass_map_path: String::new(),
            player_direction: Direction::Forward,
            player_position: [200.0, 200.0].into(),
        };

        OverworldScreen {
            room: Room::new(room_creation_params, global_resource_storage),
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
