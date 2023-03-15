use bevy::prelude::*;
use bevyterm::TerminalPlugin;

fn main() {
    App::new()
        .add_plugin(TerminalPlugin)
        .add_startup_system(hello_world)
    .run();
}


fn hello_world() {
    println!("hello world");
}
