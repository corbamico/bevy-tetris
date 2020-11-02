use crate::bricks::{Board, BrickShape, Dot};
use crate::tetrom::{BrickFallingStopEvent, BrickRotateEvent};
use bevy::prelude::*;
use std::time::Duration;

struct KeyboardTimer(Timer);

pub struct KeyboardPlugin;
impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(KeyboardTimer(Timer::new(Duration::from_millis(100), true)))
            .add_system(handle_keyboard.system());
    }
}

fn handle_keyboard(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<KeyboardTimer>,
    mut events: ResMut<Events<BrickRotateEvent>>,
    mut falling_events: ResMut<Events<BrickFallingStopEvent>>,
    board: Res<Board>,
    mut bricks: Query<(&BrickShape, &mut Dot)>,
) {
    timer.0.tick(time.delta_seconds);
    if !timer.0.finished {
        return;
    }
    for (brick_shape, mut dot) in &mut bricks.iter() {
        if keyboard.pressed(KeyCode::Right) {
            let next_dot = dot.right();
            if board.valid_brickshape(&brick_shape, &next_dot) {
                dot.move_right();
            }
        } else if keyboard.pressed(KeyCode::Left) {
            let next_dot = dot.left();
            if board.valid_brickshape(&brick_shape, &next_dot) {
                dot.move_left();
            }
        } else if keyboard.pressed(KeyCode::Down) {
            //accerlate
            let next_dot = dot.down();
            if board.valid_brickshape(&brick_shape, &next_dot) {
                dot.move_down();
            } else {
                falling_events.send(BrickFallingStopEvent);
            }
        } else if keyboard.pressed(KeyCode::Up) {
            let next_brick = brick_shape.rotate();
            if board.valid_brickshape(&next_brick, &dot) {
                events.send(BrickRotateEvent);
            }
        }
    }
}
