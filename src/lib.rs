use bevy::prelude::*;

pub mod components;
pub mod error_handling;
mod window;


pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {

    fn is_unique(&self) -> bool {
        true
    }

    fn build(&self, app: &mut App) {
        app
            .add_plugin(window::WindowPlugin)
        ;
    }
}

