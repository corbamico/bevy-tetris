use crate::bricks::{Board, BrickShape, Dot};
use crate::consts::{BOARD_Y_VALIDE, TIMER_FALLING_SECS, TIMER_KEY_SECS};
use bevy::prelude::*;
struct KeyboardTimer(Timer);
struct FallingTimer(Timer);
pub enum Movements {
    None,
    MoveTo(Dot),
    RotateTo(Dot),
    StopTo(Dot),
}
///BrickMoveRes use for [handle_keyboard] bring info to [brick_movement_handle]
pub struct BrickMoveRes(pub Movements);
pub struct KeyboardPlugin;
impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(KeyboardTimer(Timer::from_seconds(TIMER_KEY_SECS, true)))
            .add_resource(FallingTimer(Timer::from_seconds(TIMER_FALLING_SECS, true)))
            .add_resource(BrickMoveRes(Movements::None))
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
    let mut falling_dot = Dot(0, 0);

    if falling_timer.0.finished {
        for (brick_shape, dot) in &mut bricks.iter() {
            let next_position = dot.down();
            //BUG: initial brick out of Y
            if board.valid_brickshape(brick_shape, &next_position) {
                movement.0 = Movements::MoveTo(next_position);
                falling_dot.move_down();
            } else {
                movement.0 = Movements::StopTo(*dot);
                return;
            }
        }
    }
    //We should handle both input of Timer(Falling) and Timer(Keyboard)
    //otherice user will feel no response for keyboard.
    // let mut key = KeyCode::A;
    // if keyboard.pressed(KeyCode::Right) {
    //     key = KeyCode::Right;
    // } else if keyboard.pressed(KeyCode::Left) {
    //     key = KeyCode::Left;
    // } else if keyboard.pressed(KeyCode::Down) {
    //     key = KeyCode::Down;
    // } else if keyboard.just_pressed(KeyCode::Up) {
    //     key = KeyCode::Up;
    // }

    if keyboard_timer.0.finished {
        for (brick_shape, dot) in &mut bricks.iter() {
            if keyboard.pressed(KeyCode::Right) {
                let next_dot = dot.right().with_orignal_dot(&falling_dot);
                if board.valid_brickshape(&brick_shape, &next_dot) {
                    movement.0 = Movements::MoveTo(next_dot);
                }
            } else if keyboard.pressed(KeyCode::Left) {
                let next_dot = dot.left().with_orignal_dot(&falling_dot);
                if board.valid_brickshape(&brick_shape, &next_dot) {
                    movement.0 = Movements::MoveTo(next_dot);
                }
            } else if keyboard.pressed(KeyCode::Down) {
                //accerlate
                let next_dot = dot.down().with_orignal_dot(&falling_dot);
                if board.valid_brickshape(&brick_shape, &next_dot) {
                    movement.0 = Movements::MoveTo(next_dot);
                } else {
                    movement.0 = Movements::StopTo(dot.with_orignal_dot(&falling_dot));
                }
            } else if keyboard.pressed(KeyCode::Up) {
                let next_brick = brick_shape.rotate();
                let next_dot = dot.with_orignal_dot(&falling_dot);

                if board.valid_brickshape(&next_brick, &next_dot) {
                    movement.0 = Movements::RotateTo(next_dot);
                }
            } else if keyboard.pressed(KeyCode::Space) {
                let mut next_dot = dot.with_orignal_dot(&falling_dot);
                next_dot.move_down();
                for _ in 0..BOARD_Y_VALIDE {
                    if !board.valid_brickshape(brick_shape, &next_dot) {
                        movement.0 = Movements::StopTo(next_dot.up());
                        return;
                    }
                    next_dot.move_down();
                }
            }
        }
    }
}
