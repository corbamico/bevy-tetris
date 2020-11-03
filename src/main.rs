use bevy::prelude::*;
mod bricks;
mod consts;
mod inputs;
mod screen;
mod speeds;
mod states;
mod tetrom;
fn main() {
    use bevy::input::system::exit_on_esc_system;
    App::build()
        .add_plugin(screen::ScreenPlugin)
        .add_plugin(inputs::KeyboardPlugin)
        .add_plugin(tetrom::BrickMovingPlugin)
        .add_plugin(states::GameScorePlugin)
        .add_default_plugins()
        .add_system(exit_on_esc_system.system())
        .run();
}
