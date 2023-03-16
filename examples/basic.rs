use bevy::{prelude::*, app::AppExit};
use bevyterm::{TerminalPlugin, components::CrosstermWindowSettings};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(CrosstermWindowSettings {
            alternate_screen: false,
            ..default()
        })
        .add_plugin(TerminalPlugin)
        .add_system(hello_world)
    .run();
}


fn hello_world(mut exit: EventWriter<AppExit>) {
    println!("hello world");
    exit.send(AppExit);
}
