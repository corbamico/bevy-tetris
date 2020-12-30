use crate::consts;
use bevy::prelude::*;
struct StartScreen;
pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        use bevy::app::startup_stage;
        app.add_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: consts::WINDOWS_WIDTH,
            height: consts::WINDOWS_HEIGHT,
            ..Default::default()
        })
        .add_startup_system_to_stage(startup_stage::PRE_STARTUP, setup.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let start_handle = asset_server.load("screen.png");
    commands
        .insert_resource(Materials::new(asset_server, &mut materials))
        // .spawn(Camera2dComponents::default())
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(start_handle.into()),
            ..Default::default()
        })
        .with(StartScreen);
}

pub struct Materials {
    pub black: Handle<ColorMaterial>,
    pub background: Handle<ColorMaterial>,
    pub font: Handle<Font>,
}

impl Materials {
    pub fn new(
        asset_server: Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Materials {
            black: materials.add(Color::rgb_u8(0, 0, 0).into()),
            background: materials.add(Color::rgb_u8(158, 173, 135).into()),
            font: asset_server.load("digital7mono.ttf"),
        }
    }
}
