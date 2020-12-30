//we compile as windows app instead of console app
#![windows_subsystem = "windows"]
use bevy::prelude::*;
fn main() {
    App::build()
        //we initial windows size here:
        .add_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: 360.0,
            height: 443.0,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let start_handle = asset_server.load("screen.png");

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(start_handle.into()),
            ..Default::default()
        });
    commands.spawn(CameraUiBundle::default());
    spwan_board_dot(commands, materials);
}
fn spwan_board_dot(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let black = materials.add(Color::rgb_u8(0, 0, 0).into());
    let background = materials.add(Color::rgb_u8(158, 173, 135).into());
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|child| {
            spwan_child_dot(child, black, background);
        });
}

fn spwan_child_dot(
    commands: &mut ChildBuilder,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
) {
    commands
        .spawn(NodeBundle {
            material: black.clone(),
            style: Style {
                size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                align_self: AlignSelf::Stretch,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    material: background,
                    style: Style {
                        size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                        position: Rect {
                            left: Val::Px(2.0),
                            ..Default::default()
                        },
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        material: black,
                        style: Style {
                            size: Size::new(Val::Px(12.0), Val::Px(12.0)),
                            align_self: AlignSelf::Center,
                            position: Rect {
                                left: Val::Px(2.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}
