#[allow(clippy::module_inception)]
mod controller;
mod key_bindings;

pub use controller::{Controller, GameAction};
pub use key_bindings::KeyBindings;
