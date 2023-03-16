use bevy::{prelude::*, app::AppExit};
use bevyterm::{TerminalPlugin, event::CrosstermEvent};
use crossterm::event::{KeyCode, KeyModifiers};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(TerminalPlugin)
        .add_system(hello_world)
    .run();
}


fn hello_world(mut exit: EventWriter<AppExit>, mut events: EventReader<CrosstermEvent>) {
    println!("hello world");
    for event in events.iter() {
        if let crossterm::event::Event::Key(ke) = event.0 {
            if ke.code == KeyCode::Char('c') && ke.modifiers.contains(KeyModifiers::CONTROL) {
                exit.send(AppExit)
            }
        }
    }
}
