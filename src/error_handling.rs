use bevy::{prelude::*, app::AppExit};
use anyhow::Result;

pub fn crash_on_err( result: In<Result<()>>, mut exit: EventWriter<AppExit>) {
    if let Err(e) = result.0 {
        error!("{e}");
        exit.send(AppExit);
    }
}

pub fn log_on_err(result: In<Result<()>>) {
    if let Err(e) = result.0 {
        error!("{e}");
    }
}
