use crate::overworld::screen::OverworldScreen;
use crate::resources::{self, GlobalResourceStorage};
use crate::screen::Screen;
use crate::ui_event::UiEvent;
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use std::default::Default;

struct Underkate {
    screen: Box<dyn Screen>,
    global_resource_storage: GlobalResourceStorage,
}

impl Underkate {
    pub fn new(ctx: &mut Context) -> Self {
        let global_resource_storage = resources::make_global_storage(ctx);
        let screen = Box::new(OverworldScreen::new(&global_resource_storage));

        Underkate {
            screen,
            global_resource_storage,
        }
    }
}

impl EventHandler<GameError> for Underkate {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.screen.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.screen.draw(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, repeat: bool) {
        if repeat {
            return;
        }

        let ui_event = UiEvent::KeyDown { key, mods };
        self.screen.handle_event(ctx, ui_event);
    }

    fn key_up_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods) {
        let ui_event = UiEvent::KeyUp { key, mods };
        self.screen.handle_event(ctx, ui_event);
    }
}

pub fn run() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Underkate", "kodopp")
        .window_setup(WindowSetup::default().title("Underkate"))
        .build()?;

    let underkate = Underkate::new(&mut ctx);
    event::run(ctx, event_loop, underkate);
}
