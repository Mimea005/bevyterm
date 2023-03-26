use std::io::stdout;

use bevy::{prelude::*, app::AppExit};
use bevyterm::{TerminalPlugin, components::CrosstermWindow, event::KeyEvent, error_handling::crash_on_err};
use crossterm::{event::{KeyCode, KeyModifiers}, queue, cursor::MoveTo, style::Print};
use anyhow::Result;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(TerminalPlugin)
        .add_system(hello_world.pipe(crash_on_err))
    .run();
}


fn hello_world(mut exit: EventWriter<AppExit>, mut key_events: EventReader<KeyEvent>, win: Query<&CrosstermWindow>) -> Result<()> {
    let win = win.single();
    queue!(stdout(), MoveTo(win.width()/2, win.height()/2), Print("Hello World!"))?;
    for key in key_events.iter() {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            exit.send(AppExit)
        }
    }

    Ok(())
}
