use std::io::stdout;

use bevy::prelude::*;
use crossterm::{event::{EnableMouseCapture, DisableMouseCapture, KeyEvent}, QueueableCommand};
use anyhow::Result;

#[derive(Component, Debug)]
pub struct CrosstermWindow {
    pub(crate) height: u16,
    pub(crate) width: u16,
    pub(crate) title: Option<String>,
    pub(crate) alternate_screen: bool,
    pub(crate) mouse_capture: bool,
}

impl CrosstermWindow {
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn is_alternate_screen(&self) -> bool {
        self.alternate_screen
    }

    pub fn is_mouse_capture(&self) -> bool {
        self.mouse_capture
    }

    pub fn set_mouse_capture(&mut self, enable: bool) -> Result<()> {
        if self.mouse_capture != enable {
            match enable {
                true => stdout().queue(EnableMouseCapture)?,
                false => stdout().queue(DisableMouseCapture)?
            };
        }

        Ok(())
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title)
    }
}

#[derive(Resource)]
pub struct CrosstermWindowSettings {
    pub title: Option<String>,
    pub alternate_screen: bool,
    pub mouse_capture: bool
}

impl Default for CrosstermWindowSettings {
    fn default() -> Self {
        Self {
            title: None,
            alternate_screen: true,
            mouse_capture: true
        }
    }
}
