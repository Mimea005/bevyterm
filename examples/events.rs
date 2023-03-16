use std::io::{stdout, Write};

use anyhow::Result;
use bevy::{prelude::*, app::AppExit};
use bevyterm::{TerminalPlugin, components::{CrosstermWindowSettings, CrosstermWindow}, event::CrosstermEvent, error_handling::crash_on_err};
use crossterm::{terminal::Clear, queue, cursor::MoveTo, style::Print, event::{KeyEvent, KeyCode}};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(CrosstermWindowSettings {
            alternate_screen: true,
            title: Some(String::from("Event logs")),
            ..default()
        })
        .add_plugin(TerminalPlugin)
        .init_resource::<MaxLines>()
        .add_startup_system(setup.pipe(crash_on_err).in_base_set(StartupSet::PostStartup))
        .add_system(log_event.pipe(crash_on_err))
        .run();
}

#[derive(Resource, Default)]
struct MaxLines(u16, u16);

fn setup(win: Query<&CrosstermWindow>, mut lines: ResMut<MaxLines>) -> Result<()> {
    let win = win.get_single()?;

    lines.0 = win.height();
    lines.1 = 1;

    Ok(())
}

fn log_event(mut events: EventReader<CrosstermEvent>, mut lines: ResMut<MaxLines>, mut counter: Local<usize>, mut exit: EventWriter<AppExit>) -> Result<()> {

    *counter+=1;
    queue!(stdout(), MoveTo(0, 0), Print(format!("Counter: {}", *counter)))?;

    for event in events.iter() {
        queue!(stdout(), MoveTo(0, lines.1), Print(format!("{event:?}")))?;
        lines.1 += 1;

        match event.0 {
            crossterm::event::Event::Key(ke) => {
                if ke.code == KeyCode::Char('q') {
                    exit.send(AppExit);
                }
            },
            _ => {}
        }

        if lines.1 >= lines.0 {
            queue!(stdout(), Clear(crossterm::terminal::ClearType::All))?;
            lines.1 = 1;
        }
    }
    stdout().flush()?;

    Ok(())
}

