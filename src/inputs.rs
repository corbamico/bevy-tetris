use crate::bricks::{Board, BrickShape, Dot};
use crate::consts::{TIMER_FALLING_MILLIS, TIMER_KEYBOARD_MILLIS};
use bevy::prelude::*;
use std::time::Duration;
struct KeyboardTimer(Timer);
struct FallingTimer(Timer);

#[derive(Eq, PartialEq)]
pub enum Movements {
    None,
    Left,
    Right,
    Down,
    Rotate,
    Stop,
}

///BrickMoveRes use for [handle_keyboard] bring info to [brick_movement_handle]
pub struct BrickMoveRes(pub Movements);

pub struct KeyboardPlugin;
impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(KeyboardTimer(Timer::new(
            Duration::from_millis(TIMER_KEYBOARD_MILLIS),
            true,
        )))
        .add_resource(FallingTimer(Timer::new(
            Duration::from_millis(TIMER_FALLING_MILLIS),
            true,
        )))
        .add_resource(BrickMoveRes(Movements::Down))
        .add_system_to_stage(stage::FIRST, handle_keyboard.system());
    }
}

///handle_keyboard handle all inputs, including every tick of falling brick.
fn handle_keyboard(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut keyboard_timer: ResMut<KeyboardTimer>,
    mut falling_timer: ResMut<FallingTimer>,
    mut movement: ResMut<BrickMoveRes>,
    board: Res<Board>,
    mut bricks: Query<(&BrickShape, &Dot)>,
) {
    keyboard_timer.0.tick(time.delta_seconds);
    falling_timer.0.tick(time.delta_seconds);

    movement.0 = Movements::None;

    if falling_timer.0.finished {
        for (brick_shape, dot) in &mut bricks.iter() {
            let next_position = dot.down();
            //BUG: initial brick out of Y
            if board.valid_brickshape(brick_shape, &next_position) {
                movement.0 = Movements::Down;
            } else {
                movement.0 = Movements::Stop;
            }
        }
        //hight priority for handling tick of falling,
        //so we return.
        return;
    }

    if keyboard_timer.0.finished {
        for (brick_shape, dot) in &mut bricks.iter() {
            if keyboard.pressed(KeyCode::Right) {
                let next_dot = dot.right();
                if board.valid_brickshape(&brick_shape, &next_dot) {
                    movement.0 = Movements::Right;
                }
            } else if keyboard.pressed(KeyCode::Left) {
                let next_dot = dot.left();
                if board.valid_brickshape(&brick_shape, &next_dot) {
                    movement.0 = Movements::Left;
                }
            } else if keyboard.pressed(KeyCode::Down) {
                //accerlate
                let next_dot = dot.down();
                if board.valid_brickshape(&brick_shape, &next_dot) {
                    movement.0 = Movements::Down;
                } else {
                    movement.0 = Movements::Stop;
                }
            } else if keyboard.pressed(KeyCode::Up) {
                let next_brick = brick_shape.rotate();
                if board.valid_brickshape(&next_brick, &dot) {
                    movement.0 = Movements::Rotate;
                }
            }
        }
    }
}
