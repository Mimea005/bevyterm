use std::io::{stdout, Write};

use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};
use anyhow::{Result, bail};
use crossterm::{QueueableCommand, event::{EnableMouseCapture, DisableMouseCapture}, terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle}, queue};

use crate::components::{CrosstermWindowSettings, CrosstermWindow};
use crate::error_handling::{crash_on_err, log_on_err};

pub(crate) struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn is_unique(&self) -> bool {
        true
    }

    fn build(&self, app: &mut App) {
        app
            .init_resource::<CrosstermWindowSettings>()
            .add_startup_system(setup_terminal.pipe(crash_on_err).in_base_set(StartupSet::Startup))
            .add_systems((
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(StartupSet::PreStartupFlush),
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(StartupSet::StartupFlush),
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(StartupSet::PostStartupFlush),
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(CoreSet::FirstFlush),
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(CoreSet::PreUpdateFlush),
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(CoreSet::UpdateFlush),
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(CoreSet::PostUpdateFlush),
                restore_terminal_on_exit.pipe(log_on_err).in_base_set(CoreSet::LastFlush),
            ))
            .add_system(
                update_title.pipe(log_on_err)
            )
        ;
    }
}

fn setup_terminal(mut command: Commands, cross_settings: Res<CrosstermWindowSettings>) -> Result<()> {

    crossterm::terminal::enable_raw_mode()?;

    let (width, height) = crossterm::terminal::size()?;

    if cross_settings.mouse_capture {
        stdout().queue(EnableMouseCapture)?;
    }

    if cross_settings.alternate_screen {
        stdout().queue(EnterAlternateScreen)?;
    }

    if let Some(title) = cross_settings.title.clone() {
        stdout().queue(SetTitle(title))?;
    }

    command.spawn((
        CrosstermWindow {
            title: cross_settings.title.clone(),
            width, height,
            mouse_capture: cross_settings.mouse_capture,
            alternate_screen: cross_settings.alternate_screen
        },
        PrimaryWindow
    ));

    Ok(())
}

fn update_title(window: Query<&CrosstermWindow, Changed<CrosstermWindow>>) -> Result<()> {
    if let Some(window) = window.iter().last() {
        match &window.title {
            Some(title) => queue!(stdout(), SetTitle(title))?,
            None => {
                queue!(stdout(), SetTitle(""))?;
                bail!("Removed title");
            }
        }
    }

    Ok(())
}

fn restore_terminal_on_exit(exit: EventReader<AppExit>, window: Query<&CrosstermWindow>) -> Result<()> {

    if exit.is_empty() {
        return Ok(());
    }

    if let Ok(window) = window.get_single() {
        if window.is_mouse_capture() {
            stdout().queue(DisableMouseCapture)?;
        }
        if window.is_alternate_screen() {
            stdout().queue(LeaveAlternateScreen)?;
        }

        stdout().flush()?;
        crossterm::terminal::disable_raw_mode()?;
    }

    Ok(())
}
