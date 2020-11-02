use crate::bricks::*;
use crate::consts::{
    BOARD_BOTTOM_PX, BOARD_LEFT_PX, BRICK_START_DOT, DOT_WIDTH_PX, NEXT_BRICK_BOTTOM_PX,
    NEXT_BRICK_LEFT_PX, SCORE_PER_DELETE, SCORE_PER_DROP, STRING_GAME_OVER,
};
use crate::states::{GameText, LinesRes, ScoreRes};
use bevy::prelude::*;
use std::time::Duration;

pub struct BlackMaterial(Handle<ColorMaterial>);
pub struct BackgroundMaterial(Handle<ColorMaterial>);

//for Resource
struct BrickNext {
    curr: BrickShape,
    next: BrickShape,
}
//for Tag in screen
struct BrickNextTag;

struct DotInBoard;

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

struct FallingTimer(Timer);
pub struct BrickFallingStopEvent;
pub struct BrickRotateEvent;

pub struct GameFallingPlugin;
impl Plugin for GameFallingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FallingTimer(Timer::new(Duration::from_millis(300), true)))
            .add_startup_system(setup.system())
            .add_system(dot_screen_tranlation.system())
            .add_system(brick_falling.system())
            .add_system(brick_despawn_handle.system())
            .add_event::<BrickFallingStopEvent>()
            .add_event::<BrickRotateEvent>()
            .add_event::<CheckCleanLineEvent>()
            .add_event::<NewBrickEvent>()
            .add_event::<CheckGameOverEvent>()
            .add_stage_after(stage::UPDATE, "checking")
            .add_system_to_stage("checking", check_clean_line.system())
            .add_system_to_stage("checking", check_game_over.system())
            .add_system_to_stage("checking", generate_new_brick.system());
        //Checking must be handled after 'brick_despawn_handle'
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let black = materials.add(Color::rgb_u8(0, 0, 0).into());
    let background = materials.add(Color::rgb_u8(100, 109, 84).into());
    //let brick = BrickShape::rand();
    let brick_next = BrickNext::new();

    // spwan_brick(&mut commands, black, background, brick, &BRICK_START_DOT);
    // spwan_brick_next(&mut commands, black, background, brick_next.curr);

    commands
        .insert_resource(Board::default())
        .insert_resource(BlackMaterial(black))
        .insert_resource(BackgroundMaterial(background))
        .insert_resource(brick_next);

    spwan_board_dot(&mut commands, black, background, &Dot(9, 0));
}

fn dot_screen_tranlation(mut q: Query<(&Dot, &mut Style)>) {
    for (dot, mut style) in &mut q.iter() {
        if style.position_type == PositionType::Absolute {
            style.position = Rect {
                left: Val::Px(dot_to_screen_x(dot)),
                bottom: Val::Px(dot_to_screen_y(dot)),
                ..Default::default()
            }
        }
    }
}

fn brick_falling(
    time: Res<Time>,
    mut timer: ResMut<FallingTimer>,
    borad: Res<Board>,
    mut events: ResMut<Events<BrickFallingStopEvent>>,
    mut q: Query<(&BrickShape, &mut Dot)>,
) {
    timer.0.tick(time.delta_seconds);

    if !timer.0.finished {
        return;
    }
    for (brick_shape, mut dot) in &mut q.iter() {
        let next_position = dot.down();
        //BUG: initial brick out of Y
        if borad.valid_brickshape(brick_shape, &next_position) {
            dot.move_down();
        } else {
            events.send(BrickFallingStopEvent);
        }
    }
}

fn brick_despawn_handle(
    mut commands: Commands,
    mut reader: Local<EventReader<BrickFallingStopEvent>>,
    events: Res<Events<BrickFallingStopEvent>>,

    mut rotate_reader: Local<EventReader<BrickRotateEvent>>,
    rotate_event: Res<Events<BrickRotateEvent>>,
    mut event_sender: ResMut<Events<CheckCleanLineEvent>>,

    mut board: ResMut<Board>,
    mut score_res: ResMut<ScoreRes>,

    black: Res<BlackMaterial>,
    background: Res<BackgroundMaterial>,
    mut bricks: Query<(Entity, &BrickShape, &Dot)>,
) {
    if reader.iter(&events).next().is_some() {
        score_res.0 += SCORE_PER_DROP;

        //step 1. fix this brick to board
        for (entity, brick_shape, orig) in &mut bricks.iter() {
            //notes:
            //despawn brick and spwan dots with components(DotInBoard)
            spwan_brick_as_dot(&mut commands, black.0, background.0, *brick_shape, orig);

            commands.despawn_recursive(entity);

            //commands.despawn_recursive(entity);
            board.occupy_brickshape(brick_shape, orig);
        }

        //step 2. check cleaning lines
        event_sender.send(CheckCleanLineEvent);

        //step 3. check game over  in trigger in Event CheckCleanLineEvent
        //step 4. generate random new brick in trigger in Event CheckGameOverEvent
    }
    if rotate_reader.iter(&rotate_event).next().is_some() {
        for (entity, brick_shape, dot) in &mut bricks.iter() {
            let next_shape = brick_shape.rotate();

            commands.despawn_recursive(entity);

            //commands.despawn_recursive(entity);
            spwan_brick(&mut commands, black.0, background.0, next_shape, dot);
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

//Only for fix brick to board
//and easy for deleting one dot by dot or lines
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

struct CheckCleanLineEvent;
struct CheckGameOverEvent;
pub struct NewBrickEvent;

fn check_clean_line(
    mut commands: Commands,
    mut reader: Local<EventReader<CheckCleanLineEvent>>,
    events: Res<Events<CheckCleanLineEvent>>,
    mut event_sender: ResMut<Events<CheckGameOverEvent>>,
    mut board: ResMut<Board>,
    mut score_res: ResMut<ScoreRes>,
    mut lines_res: ResMut<LinesRes>,
    mut query: Query<(Entity, &mut Dot, &mut DotInBoard)>,
) {
    if reader.iter(&events).next().is_some() {
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
        event_sender.send(CheckGameOverEvent);
    }
}

fn check_game_over(
    mut commands: Commands,
    mut reader: Local<EventReader<CheckGameOverEvent>>,
    events: Res<Events<CheckGameOverEvent>>,
    mut event_sender: ResMut<Events<NewBrickEvent>>,
    mut board: ResMut<Board>,
    mut query: Query<(&GameText, &mut Text)>,
) {
    if reader.iter(&events).next().is_some() {
        if board.game_over() {
            //Game-Over!!
            println!("Game Over!");
            for (_, mut text) in &mut query.iter() {
                text.value = STRING_GAME_OVER.to_string();
            }
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
