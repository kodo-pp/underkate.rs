use crate::overworld::screen::OverworldScreen;
use crate::resources::{self, GlobalResourceStorage};
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
use crate::default_runtime::DefaultRuntime;
use crate::script::{EventHandleGenerator, ScriptHandleGenerator};

struct Underkate {
    screen: Arc<Mutex<RefCell<dyn Screen>>>,
    global_resource_storage: Arc<GlobalResourceStorage>,
    runtime: Arc<Mutex<RefCell<DefaultRuntime>>>,
    event_handle_generator: EventHandleGenerator,
    script_handle_generator: ScriptHandleGenerator,
}

impl Underkate {
    pub fn new(ctx: &mut Context) -> Self {
        let global_resource_storage = Arc::new(resources::make_global_storage(ctx));
        let screen = Arc::new(Mutex::new(RefCell::new(OverworldScreen::new(
            &global_resource_storage,
        ))));
        let runtime = Arc::new(Mutex::new(RefCell::new(DefaultRuntime::new())));
        let event_handle_generator = EventHandleGenerator::new();
        let script_handle_generator = ScriptHandleGenerator::new();

        Underkate {
            screen,
            global_resource_storage,
            runtime,
            event_handle_generator,
            script_handle_generator,
        }
    }
}

impl EventHandler<GameError> for Underkate {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.screen.lock().unwrap().borrow_mut().update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.screen.lock().unwrap().borrow_mut().draw(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, repeat: bool) {
        if repeat {
            return;
        }

        let ui_event = UiEvent::KeyDown { key, mods };
        self.screen
            .lock()
            .unwrap()
            .borrow_mut()
            .handle_event(ctx, ui_event);
    }

    fn key_up_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods) {
        let ui_event = UiEvent::KeyUp { key, mods };
        self.screen
            .lock()
            .unwrap()
            .borrow_mut()
            .handle_event(ctx, ui_event);
    }
}

pub fn run() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Underkate", "kodopp")
        .window_setup(WindowSetup::default().title("Underkate"))
        .build()?;

    // TODO: loading screen.

    let underkate = Underkate::new(&mut ctx);
    event::run(ctx, event_loop, underkate);
}
