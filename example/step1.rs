//we compile as windows app instead of console app
#![windows_subsystem = "windows"]
use bevy::prelude::*;
fn main(){
    App::build()
    //we initial windows size here:
    .add_resource(WindowDescriptor {
     title: "Tetris".to_string(),width: 360,height: 443,..Default::default()})
    .add_startup_system(setup.system())
    .add_plugins(DefaultPlugins)
    .run();
}
fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    let start_handle = asset_server.load("screen.png");
    commands.spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(start_handle.into()),
            ..Default::default()
        });
}
