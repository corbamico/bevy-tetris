use crate::bricks::*;
use crate::consts::{
    BOARD_BOTTOM_PX, BOARD_LEFT_PX, BRICK_START_DOT, DOT_WIDTH_PX, NEXT_BRICK_BOTTOM_PX,
    NEXT_BRICK_LEFT_PX, SCORE_PER_DELETE, SCORE_PER_DROP, STRING_GAME_OVER,
};
use crate::inputs::{BrickMoveRes, Movements};
use crate::states::{GameStage, GameState, GameText, LinesRes, ScoreRes};
use bevy::prelude::*;

pub struct BlackMaterial(Handle<ColorMaterial>);
pub struct BackgroundMaterial(Handle<ColorMaterial>);

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
            .add_system_to_stage(stage::PRE_UPDATE, handle_brick_movement.system())
            .add_system_to_stage(stage::UPDATE, check_clean_line.system())
            .add_system_to_stage(stage::UPDATE, check_game_over.system())
            .add_system_to_stage(stage::UPDATE, generate_new_brick.system())
            .add_system_to_stage(stage::UPDATE, translate_coordinate.system());
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let black = materials.add(Color::rgb_u8(0, 0, 0).into());
    let background = materials.add(Color::rgb_u8(100, 109, 84).into());
    let brick_next = BrickNext::new();

    commands
        .insert_resource(Board::default())
        .insert_resource(BlackMaterial(black))
        .insert_resource(BackgroundMaterial(background))
        .insert_resource(brick_next);

    //Test Point:
    //spwan_board_dot(&mut commands, black, background, &Dot(9, 0));
}

fn translate_coordinate(mut q: Query<(Changed<Dot>, &mut Style)>) {
    for (dot, mut style) in &mut q.iter() {
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
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut score_res: ResMut<ScoreRes>,
    movement: Res<BrickMoveRes>,

    black: Res<BlackMaterial>,
    background: Res<BackgroundMaterial>,
    mut bricks: Query<(Entity, &BrickShape, &mut Dot)>,
) {
    match movement.0 {
        Movements::None => {}
        Movements::MoveTo(next_dot) => {
            for (_, _, mut dot) in &mut bricks.iter() {
                *dot = next_dot;
            }
        }
        Movements::RotateTo(next_dot) => {
            for (entity, brick_shape, _) in &mut bricks.iter() {
                let next_shape = brick_shape.rotate();
                commands.despawn_recursive(entity);
                spwan_brick(&mut commands, black.0, background.0, next_shape, &next_dot);
            }
        }
        Movements::StopTo(next_dot) => {
            score_res.0 += SCORE_PER_DROP;
            //step 1. fix this brick to board
            for (entity, brick_shape, _) in &mut bricks.iter() {
                //notes:
                //despawn brick and spwan dots with components(DotInBoard)
                spwan_brick_as_dot(
                    &mut commands,
                    black.0,
                    background.0,
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
    for i in 0..4 {
        spwan_board_dot(
            commands,
            black,
            background,
            &brick.dots[i].with_orignal_dot(orig),
        );
    }
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
        .spawn(NodeComponents {
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
            spwan_child_dot(child, black, background, &brick.dots[0]);
            spwan_child_dot(child, black, background, &brick.dots[1]);
            spwan_child_dot(child, black, background, &brick.dots[2]);
            spwan_child_dot(child, black, background, &brick.dots[3]);
        });
}

fn spwan_board_dot(
    commands: &mut Commands,
    black: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    dot: &Dot,
) {
    commands
        .spawn(NodeComponents {
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
        .spawn(NodeComponents {
            material: black,
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
                .spawn(NodeComponents {
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
                    parent.spawn(NodeComponents {
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
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut score_res: ResMut<ScoreRes>,
    mut lines_res: ResMut<LinesRes>,
    movement: Res<BrickMoveRes>,
    mut query: Query<(Entity, &mut Dot, &mut DotInBoard)>,
) {
    if let Movements::StopTo(_) = movement.0 {
        //deleted_lines should be sorted desc.
        let deleted_lines = board.get_clean_lines();
        for line in deleted_lines {
            score_res.0 += SCORE_PER_DELETE;
            lines_res.0 += 1;

            for (entity, mut dot, _) in &mut query.iter() {
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
    mut commands: Commands,
    mut event_sender: ResMut<Events<NewBrickEvent>>,
    movement: Res<BrickMoveRes>,
    board: ResMut<Board>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&GameText, &mut Text)>,
    mut dots: Query<(Entity, &Dot, &DotInBoard)>,
) {
    if let Movements::StopTo(_) = movement.0 {
        if board.game_over() {
            //Game-Over
            //step 1.clear lines for show "game over" string
            for (entity, dot, _) in &mut dots.iter() {
                if dot_in_game_text(dot) {
                    commands.despawn_recursive(entity);
                }
            }
            //step 2.show text "game over"
            for (_, mut text) in &mut query.iter() {
                text.value = STRING_GAME_OVER.to_string();
            }
            //step 3.change state
            game_state.0 = GameStage::GameOver
        } else {
            event_sender.send(NewBrickEvent);
        }
    }
}

fn generate_new_brick(
    mut commands: Commands,
    mut reader: Local<EventReader<NewBrickEvent>>,
    events: Res<Events<NewBrickEvent>>,
    black: Res<BlackMaterial>,
    background: Res<BackgroundMaterial>,
    mut brick_next: ResMut<BrickNext>,
    mut query: Query<(Entity, &BrickNextTag)>,
) {
    if reader.iter(&events).next().is_some() {
        spwan_brick(
            &mut commands,
            black.0,
            background.0,
            brick_next.curr,
            &BRICK_START_DOT,
        );
        brick_next.next();
        for (entity, _) in &mut query.iter() {
            commands.despawn_recursive(entity);
        }
        spwan_brick_next(&mut commands, black.0, background.0, brick_next.curr);
    }
}

fn dot_in_game_text(dot: &Dot) -> bool {
    let vec = vec![12, 13, 14];
    vec.contains(&dot.1)
}
