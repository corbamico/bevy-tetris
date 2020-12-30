//we compile as windows app instead of console app
#![windows_subsystem = "windows"]
use bevy::prelude::*;

// Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(3, 1)]},
// Brick{dots:[Dot(2, 0), Dot(2, 1), Dot(2, 2), Dot(2, 3)]}
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
    mut commands: &mut Commands,
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

    let black = materials.add(Color::rgb_u8(0, 0, 0).into());
    let background = materials.add(Color::rgb_u8(158, 173, 135).into());

    let brick = Brick {
        dots: [Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(3, 1)],
    };
    spwan_brick_at(&mut commands, black, background, brick, 0.0, 0.0);
}

fn spwan_brick_at(
    commands: &mut Commands,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    brick: Brick,
    x: f32,
    y: f32,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(x),
                    bottom: Val::Px(y),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|child| {
            (0..4).for_each(|i| {
                spwan_child_dot(child, black.clone(), background.clone(), &brick.dots[i])
            });
        });
}

fn spwan_child_dot(
    commands: &mut ChildBuilder,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    dot: &Dot,
) {
    commands
        .spawn(NodeBundle {
            material: black.clone(),
            style: Style {
                size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(dot_to_brick_x(dot)),
                    bottom: Val::Px(dot_to_brick_y(dot)),
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

#[derive(Copy, Clone)]
pub struct Brick {
    pub dots: [Dot; 4],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Dot(pub i8, pub i8);

fn dot_to_brick_x(dot: &Dot) -> f32 {
    21.0 * dot.0 as f32
}
fn dot_to_brick_y(dot: &Dot) -> f32 {
    21.0 * dot.1 as f32
}
