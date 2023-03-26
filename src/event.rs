use std::time::Duration;

use bevy::prelude::*;
use anyhow::Result;

use crate::{error_handling::log_on_err, components::CrosstermWindow};

/// All events collected from crossterm.
/// (This includes: KeyPresses, MouseEvents and WindowEvents)
#[derive(Debug, Deref, PartialEq, Eq, Hash, Clone)]
pub struct CrosstermEvent(crossterm::event::Event);

/// Groups window related crossterm events
/// to its own type
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum WindowEvent {
    FocusGained,
    FocusLost,
    Resize(u16, u16)
}

/// Container for crossterm's KeyEvent
#[derive(Debug, Deref, PartialEq, Eq, Hash, Clone)]
pub struct KeyEvent(crossterm::event::KeyEvent);

/// Container for crossterm's MouseEvent
#[derive(Debug, Deref, PartialEq, Eq, Hash, Clone)]
pub struct MouseEvent(crossterm::event::MouseEvent);

/// Container for crossterm's Paste event
#[cfg(feature = "bracketed-paste")]
#[derive(Debug, Deref, PartialEq, Eq, Hash, Clone)]
pub struct PasteEvent(String);

pub(crate) struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Events<CrosstermEvent>>()
            .init_resource::<Events<WindowEvent>>()
            .init_resource::<Events<KeyEvent>>()
            .init_resource::<Events<MouseEvent>>()
            .add_system(poll_events.pipe(log_on_err).in_base_set(CoreSet::First))
            .add_system(update_window.pipe(log_on_err).in_base_set(CoreSet::PreUpdate))
        ;
    }
}

fn poll_events(
    mut all_events: ResMut<Events<CrosstermEvent>>,
    mut window_events: ResMut<Events<WindowEvent>>,
    mut key_events: ResMut<Events<KeyEvent>>,
    mut mouse_events: ResMut<Events<MouseEvent>>,
    #[cfg(feature = "bracketed-paste")]
    mut paste_events: ResMut<Events<PasteEvent>>
) -> Result<()> {
    // clear previous events
    all_events.clear();
    window_events.clear();
    key_events.clear();
    mouse_events.clear();

    // check for new events
    // Not sure if this method is stable, may be blocking unintentionally
    while crossterm::event::poll(Duration::ZERO)? {
        let event = crossterm::event::read()?;
        all_events.send(CrosstermEvent(event.clone()));

        match event {
            crossterm::event::Event::FocusGained => window_events.send(WindowEvent::FocusGained),
            crossterm::event::Event::FocusLost => window_events.send(WindowEvent::FocusLost),
            crossterm::event::Event::Resize(col, row) => window_events.send(WindowEvent::Resize(col, row)),
            crossterm::event::Event::Key(k) => key_events.send(KeyEvent(k)),
            crossterm::event::Event::Mouse(m) => mouse_events.send(MouseEvent(m)),
            #[cfg(feature = "bracketed-paste")]
            crossterm::event::Event::Paste(txt) => paste_events.send(PasteEvent(txt)),
            _ => unreachable!("All events should have been exhausted!")
        }
    }

    Ok(())
}

fn update_window(mut window: Query<&mut CrosstermWindow>, mut events: EventReader<WindowEvent>) -> Result<()>{
    let mut window = window.get_single_mut()?;

    for event in events.iter() {
        match event {
            WindowEvent::Resize(col, row) => (window.width, window.height) = (*col, *row),
            _ => {}
        }
    }

    Ok(())
}
