use std::{io::{stdout, Write}, time::Duration};

use anyhow::Result;
use bevy::{prelude::*, app::{AppExit, ScheduleRunnerSettings}, reflect::erased_serde::__private::serde::de::IntoDeserializer};
use bevyterm::{TerminalPlugin, components::{CrosstermWindowSettings, CrosstermWindow}, event::{CrosstermEvent, KeyEvent, WindowEvent, MouseEvent}, error_handling::crash_on_err};
use crossterm::{terminal::{Clear, ClearType}, queue, cursor::MoveTo, style::Print, event::{KeyCode, KeyModifiers}};

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(32)))
        .add_plugins(MinimalPlugins)
        .insert_resource(CrosstermWindowSettings {
            alternate_screen: true,
            title: Some(String::from("Event logs")),
            ..default()
        })
        .add_plugin(TerminalPlugin)
        .init_resource::<Lines>()
        .add_startup_system(setup.pipe(crash_on_err).in_base_set(StartupSet::PostStartup))
        .add_system(log_event.pipe(crash_on_err))
        .run();
}

#[derive(Resource, Default)]
struct Lines {
    max_lines: u16,
    key_lines: Vec<String>,
    window_lines: Vec<String>,
    mouse_lines: String,
}

fn setup(win: Query<&CrosstermWindow>, mut lines: ResMut<Lines>) -> Result<()> {
    let win = win.get_single()?;

    lines.max_lines = win.height();

    Ok(())
}

fn log_event(
    mut key_events: EventReader<KeyEvent>,
    mut window_events: EventReader<WindowEvent>,
    mut mouse_events: EventReader<MouseEvent>,
    window: Query<&CrosstermWindow>,
    mut lines: ResMut<Lines>,
    mut counter: Local<usize>,
    mut exit: EventWriter<AppExit>
) -> Result<()> {

    let window = window.single();

    *counter+=1;

    for event in key_events.iter() {
        if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
            exit.send(AppExit);
            return Ok(());
        }

        lines.key_lines.push(format!("code: {:?} mod: {:?}", event.code, event.modifiers));

        if lines.key_lines.len() >= lines.max_lines.into() {
            lines.key_lines.clear();
        }
    }

    for event in window_events.iter() {
        lines.window_lines.push(format!("{event:?}"));

        if lines.window_lines.len() >= lines.max_lines.into() {
            lines.window_lines.clear();
        }
    }

    match mouse_events.iter().last() {
        Some(event) => lines.mouse_lines = format!("{:?} ({},{})", event.kind, event.column, event.row),
        None => lines.mouse_lines = "".into()
    }

    // Clearly looks unoptimized
    // however does not result in any visible stutter
    // so stays until render part of library has been written

    queue!(stdout(),Clear(ClearType::All))?;
    queue!(stdout(), MoveTo(0, 0), Print(format!("Counter: {}", *counter)))?;

    lines.key_lines.iter()
                   .enumerate()
                   .try_for_each(|(i, s)| queue!(stdout(), MoveTo(0, i as u16+1), Print(s)))?;
    lines.window_lines.iter()
                      .enumerate()
                      .try_for_each(|(i, s)| queue!(stdout(), MoveTo(window.width() * 1/3, i as u16+1), Print(s)))?;

    queue!(stdout(), MoveTo(window.width() * 2/3, window.height()/2), Print(lines.mouse_lines.clone()))?;

    stdout().flush()?;
    Ok(())
}
