use ggez::input::keyboard::{KeyCode, KeyMods};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UiEvent {
    KeyDown { key: KeyCode, mods: KeyMods },
    KeyUp { key: KeyCode, mods: KeyMods },
}
