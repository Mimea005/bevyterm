use std::time::Duration;

use bevy::prelude::*;
use anyhow::Result;

use crate::error_handling::log_on_err;

pub(crate) struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Events<CrosstermEvent>>()
            .add_system(poll_input.pipe(log_on_err).in_base_set(CoreSet::First))
        ;
    }
}

#[derive(Debug, Deref)]
pub struct CrosstermEvent(pub crossterm::event::Event);

fn poll_input(mut events: ResMut<Events<CrosstermEvent>>) -> Result<()> {
    // clear previous events
    events.clear();

    // check for new events
    // Not sure if this method is stable, may be blocking unintentionally
    while crossterm::event::poll(Duration::ZERO)? {
        events.send(CrosstermEvent(crossterm::event::read()?));
    }

    Ok(())
}
