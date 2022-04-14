use crate::default_runtime::DefaultRuntime;
use crate::game_context::GameContext;
use crate::overworld::room::{CreationParams, Room};
use crate::overworld::screen::OverworldScreen;
use crate::resources::{self, ResourceStorageCloneExt};
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use std::cell::RefCell;
use std::default::Default;
use std::sync::{Arc, Mutex};

struct Underkate {
    game_context: GameContext,
}

impl Underkate {
    pub fn new(ctx: &mut Context) -> Self {
        let global_resource_storage = Arc::new(resources::make_global_storage(ctx));
        let runtime = Arc::new(Mutex::new(RefCell::new(DefaultRuntime::new())));
        let overworld_screen = Arc::new(Mutex::new(RefCell::new(OverworldScreen::new())));

        let game_context = GameContext {
            global_resource_storage,
            runtime,
            overworld_screen,
        };

        let starting_room = Room::new(
            CreationParams::from_partial(
                game_context.global_resource_storage.get_cloned("home/room"),
                "_",
            ),
            game_context.global_resource_storage.as_ref(),
        );
        game_context
            .overworld_screen
            .lock()
            .unwrap()
            .borrow_mut()
            .load_room(game_context.as_context_ref(), starting_room);
        game_context
            .runtime
            .lock()
            .unwrap()
            .borrow_mut()
            .update(ctx);

        Underkate {
            game_context,
        }
    }
}

impl EventHandler<GameError> for Underkate {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game_context
            .overworld_screen
            .lock()
            .unwrap()
            .borrow_mut()
            .update(ctx, self.game_context.as_context_ref())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.game_context
            .overworld_screen
            .lock()
            .unwrap()
            .borrow_mut()
            .draw(ctx, self.game_context.as_context_ref())?;
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, repeat: bool) {
        if repeat {
            return;
        }

        let ui_event = UiEvent::KeyDown { key, mods };
        self.game_context
            .overworld_screen
            .lock()
            .unwrap()
            .borrow_mut()
            .handle_event(ctx, self.game_context.as_context_ref(), ui_event);
    }

    fn key_up_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods) {
        let ui_event = UiEvent::KeyUp { key, mods };
        self.game_context
            .overworld_screen
            .lock()
            .unwrap()
            .borrow_mut()
            .handle_event(ctx, self.game_context.as_context_ref(), ui_event);
    }
}

pub fn run() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Underkate", "kodopp")
        .window_setup(WindowSetup::default().title("Underkate"))
        .build()?;

    // TODO: loading overworld_screen.

    let underkate = Underkate::new(&mut ctx);
    event::run(ctx, event_loop, underkate);
}
