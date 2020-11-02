use crate::consts;
use bevy::prelude::*;
struct StartScreen;
pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: consts::WINDOWS_WIDTH,
            height: consts::WINDOWS_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let start_handle = asset_server.load("assets/screen.png").unwrap();
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(start_handle.into()),
            ..Default::default()
        })
        .with(StartScreen);
}
