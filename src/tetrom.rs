use crate::bricks::*;
use crate::consts::*;
use crate::inputs::{BrickMoveRes, FallingTimer, Movements};
use crate::screen::Materials;
use crate::states::{GameData, GameState, GameText};
use bevy::prelude::*;

//for Resource
struct BrickNext {
    curr: BrickShape,
    next: BrickShape,
}
//for Tag in screen
struct BrickNextTag;

pub struct DotInBoard;

impl BrickNext {
    fn new() -> Self {
        BrickNext {
            curr: BrickShape::rand(),
            next: BrickShape::rand(),
        }
    }
    fn next(&mut self) {
        self.curr = self.next;
        self.next = BrickShape::rand();
    }
}
pub struct BrickMovingPlugin;
impl Plugin for BrickMovingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_event::<NewBrickEvent>()
            .add_system_to_stage(stage::EVENT, handle_brick_movement.system())
            .add_system_to_stage(stage::UPDATE, check_clean_line.system())
            .add_system_to_stage(stage::UPDATE, check_game_over.system())
            .add_system_to_stage(stage::UPDATE, generate_new_brick.system())
            .add_system_to_stage(stage::UPDATE, translate_coordinate.system());
    }
}

fn setup(commands: &mut Commands) {
    let brick_next = BrickNext::new();

    commands
        .insert_resource(Board::default())
        .insert_resource(brick_next);

    //We can draw test dot in screen here:
    //spwan_board_dot(&mut commands, black, background, &Dot(9, 0));
}

// fn translate_coordinate(mut q: Query<(Changed<Dot>, &mut Style)>) {
//     for (dot, mut style) in &mut q.iter_mut() {
//         if style.position_type == PositionType::Absolute {
//             style.position = Rect {
//                 left: Val::Px(dot_to_screen_x(&dot)),
//                 bottom: Val::Px(dot_to_screen_y(&dot)),
//                 ..Default::default()
//             }
//         }
//     }
// }
fn translate_coordinate(mut q: Query<(&Dot, &mut Style),Changed<Dot>>) {
    for (dot, mut style) in &mut q.iter_mut() {
        if style.position_type == PositionType::Absolute {
            style.position = Rect {
                left: Val::Px(dot_to_screen_x(&dot)),
                bottom: Val::Px(dot_to_screen_y(&dot)),
                ..Default::default()
            }
        }
    }
}

fn handle_brick_movement(
    mut commands: &mut Commands,
    mut board: ResMut<Board>,
    movement: Res<BrickMoveRes>,
    materials: Res<Materials>,
    mut bricks: Query<(Entity, &BrickShape, &mut Dot)>,
) {
    match movement.0 {
        Movements::None => {}
        Movements::MoveTo(next_dot) => {
            for (_, _, mut dot) in &mut bricks.iter_mut() {
                *dot = next_dot;
            }
        }
        Movements::RotateTo(next_dot) => {
            for (entity, brick_shape, _) in &mut bricks.iter_mut() {
                let next_shape = brick_shape.rotate();
                commands.despawn_recursive(entity);
                spwan_brick(
                    &mut commands,
                    materials.black.clone(),
                    materials.background.clone(),
                    next_shape,
                    &next_dot,
                );
            }
        }
        Movements::StopTo(next_dot) => {
            //step 1. fix this brick to board
            for (entity, brick_shape, _) in &mut bricks.iter_mut() {
                //notes:
                //despawn brick and spwan dots with components(DotInBoard)
                spwan_brick_as_dot(
                    &mut commands,
                    materials.black.clone(),
                    materials.background.clone(),
                    *brick_shape,
                    &next_dot,
                );
                commands.despawn_recursive(entity);
                //update game board
                board.occupy_brickshape(brick_shape, &next_dot);
            }

            //step 2. check cleaning lines
            // if Movement::Stop then check cleaning lines in function check_clean_line
        }
    }
}

fn dot_to_screen_x(dot: &Dot) -> f32 {
    BOARD_LEFT_PX + DOT_WIDTH_PX * dot.0 as f32
}
fn dot_to_screen_y(dot: &Dot) -> f32 {
    BOARD_BOTTOM_PX + DOT_WIDTH_PX * dot.1 as f32
}

fn dot_to_brick_x(dot: &Dot) -> f32 {
    DOT_WIDTH_PX * dot.0 as f32
}
fn dot_to_brick_y(dot: &Dot) -> f32 {
    DOT_WIDTH_PX * dot.1 as f32
}

fn spwan_brick(
    commands: &mut Commands,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    brick: BrickShape,
    orig: &Dot,
) {
    spwan_brick_at(
        commands,
        black,
        background,
        brick,
        dot_to_screen_x(orig),
        dot_to_screen_y(orig),
    );
    commands.with(*orig).with(brick);
}

///spwan_brick_as_dot only use for fixing brick to board
///and so it is easy for deleting one dot by dot or lines latter
fn spwan_brick_as_dot(
    commands: &mut Commands,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    brick: BrickShape,
    orig: &Dot,
) {
    let brick: Brick = brick.into();
    (0..4).for_each(|i| {
        spwan_board_dot(
            commands,
            black.clone(),
            background.clone(),
            &brick.dots[i].with_orignal_dot(orig),
        )
    });
}

fn spwan_brick_next(
    commands: &mut Commands,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    brick: BrickShape,
) {
    spwan_brick_at(
        commands,
        black,
        background,
        brick,
        NEXT_BRICK_LEFT_PX,
        NEXT_BRICK_BOTTOM_PX,
    );
    commands.with(BrickNextTag);
}
fn spwan_brick_at(
    commands: &mut Commands,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    brick: BrickShape,
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
            let brick: Brick = brick.into();
            (0..4).for_each(|i| {
                spwan_child_dot(child, black.clone(), background.clone(), &brick.dots[i])
            });
        });
}

fn spwan_board_dot(
    commands: &mut Commands,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    dot: &Dot,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(dot_to_screen_x(dot)),
                    bottom: Val::Px(dot_to_screen_y(dot)),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|child| {
            spwan_child_dot(child, black, background, &Dot::default());
        })
        .with(*dot) //dot 坐标
        .with(DotInBoard);
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

pub struct NewBrickEvent;

fn check_clean_line(
    commands: &mut Commands,
    mut board: ResMut<Board>,
    mut game_data: ResMut<GameData>,
    movement: Res<BrickMoveRes>,
    mut falling_timer: ResMut<FallingTimer>,
    mut query: Query<(Entity, &mut Dot, &mut DotInBoard)>,
) {
    if let Movements::StopTo(_) = movement.0 {
        //deleted_lines should be sorted desc.
        let deleted_lines = board.get_clean_lines();
        let len = deleted_lines.len() as u32;
        game_data.add_score(SCORE_PER_DROP);

        if len > 0 {
            if game_data.add_lines(len) {
                falling_timer.change_sceconds(game_data.get_speed());
            }
        }

        for line in deleted_lines {
            for (entity, mut dot, _) in &mut query.iter_mut() {
                //delete this line.
                if dot.1 == line {
                    commands.despawn_recursive(entity);
                } else if dot.1 > line {
                    //move one line if Y > deleted_line
                    dot.move_down();
                }
            }
        }
        board.clean_lines();
    }
}

fn check_game_over(
    commands: &mut Commands,
    mut event_sender: ResMut<Events<NewBrickEvent>>,
    movement: Res<BrickMoveRes>,
    board: ResMut<Board>,
    brick_next: Res<BrickNext>,
    mut game_data: ResMut<GameData>,
    mut query: Query<(&GameText, &mut Text)>,
    dots: Query<(Entity, &Dot, &DotInBoard)>,
) {
    if let Movements::StopTo(Dot(_, y)) = movement.0 {
        if y >= BOARD_Y_VALIDE
            || board.game_over()
            || !board.valid_brickshape(&brick_next.curr, &BRICK_START_DOT)
        {
            //Game-Over
            //step 1.clear some lines for show "Game Over" string
            for (entity, dot, _) in &mut dots.iter() {
                if dot_in_game_text(dot) {
                    commands.despawn_recursive(entity);
                }
            }
            //step 2.show text "Game Over"
            for (_, mut text) in &mut query.iter_mut() {
                text.value = STRING_GAME_OVER.to_string();
            }
            //step 3.change state
            game_data.game_state = GameState::GameOver
        } else {
            event_sender.send(NewBrickEvent);
        }
    }
}

fn generate_new_brick(
    mut commands: &mut Commands,
    mut reader: Local<EventReader<NewBrickEvent>>,
    events: Res<Events<NewBrickEvent>>,
    materials: Res<Materials>,
    mut brick_next: ResMut<BrickNext>,
    query: Query<(Entity, &BrickNextTag)>,
) {
    if reader.iter(&events).next().is_some() {
        spwan_brick(
            &mut commands,
            materials.black.clone(),
            materials.background.clone(),
            brick_next.curr,
            &BRICK_START_DOT,
        );
        brick_next.next();
        for (entity, _) in &mut query.iter() {
            commands.despawn_recursive(entity);
        }
        spwan_brick_next(
            &mut commands,
            materials.black.clone(),
            materials.background.clone(),
            brick_next.curr,
        );
    }
}

fn dot_in_game_text(dot: &Dot) -> bool {
    let vec = vec![12, 13, 14];
    vec.contains(&dot.1)
}
