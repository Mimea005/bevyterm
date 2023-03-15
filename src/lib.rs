use bevy::prelude::*;

mod runner;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {

    fn is_unique(&self) -> bool {
        true
    }

    fn build(&self, app: &mut App) {
        app
            .set_runner(runner::runner)
        ;
    }
}
