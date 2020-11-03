#![allow(dead_code)]
use crate::bricks::{Brick, Dot};
use lazy_static::*;

pub(crate) const WINDOWS_WIDTH: u32 = 360;
pub(crate) const WINDOWS_HEIGHT: u32 = 443;

pub(crate) const TEXT_SCORE_X: f32 = 248.0;
pub(crate) const TEXT_SCORE_Y: f32 = 48.0;

pub(crate) const TEXT_LINES_X: f32 = 248.0;
pub(crate) const TEXT_LINES_Y: f32 = 126.0;

pub(crate) const TEXT_LEVEL_X: f32 = 248.0;
pub(crate) const TEXT_LEVEL_Y: f32 = 202.0;

pub(crate) const TEXT_GAME_X: f32 = 50.0;
pub(crate) const TEXT_GAME_Y: f32 = 118.0;

pub(crate) const BOARD_X: i8 = 10;
pub(crate) const BOARD_Y: i8 = 23; // board is 10x20
pub(crate) const BOARD_X_Y: usize = 230; // we create 230 for more space for rotate brick.

pub(crate) const BOARD_Y_VALIDE: i8 = 20; // checking for game over

pub(crate) const BOARD_LEFT_PX: f32 = 13.0;
pub(crate) const BOARD_BOTTOM_PX: f32 = 13.0;
pub(crate) const DOT_WIDTH_PX: f32 = 21.0;

pub(crate) const NEXT_BRICK_LEFT_PX: f32 = 263.0;
pub(crate) const NEXT_BRICK_BOTTOM_PX: f32 = 100.0;

pub(crate) const BRICK_START_DOT: Dot = Dot(3, 18);

pub(crate) const BRICKS_TYPES: usize = 7;

pub(crate) const SCORE_PER_DELETE: u32 = 100;
pub(crate) const SCORE_PER_DROP: u32 = 10;

pub(crate) const STRING_GAME_START: &str = "PRESS SPACE";
pub(crate) const STRING_GAME_PLAYING: &str = "";
pub(crate) const STRING_GAME_OVER: &str = " GAME OVER \n\nPRESS SPACE";

//delay = 725 * .85 ^ level + level (ms)
pub(crate) const TIMER_FALLING_SECS: f32 = 0.725;
pub(crate) const TIMER_KEY_SECS: f32 = 0.100;

lazy_static! {
    pub static ref BRICKS_DICT: Vec<Vec<Brick>> = vec![
        //O:
        vec![Brick{dots:[Dot(1, 1), Dot(1, 2), Dot(2, 1), Dot(2, 2)]}],
        //I:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(3, 1)]},
            Brick{dots:[Dot(2, 0), Dot(2, 1), Dot(2, 2), Dot(2, 3)]}
        ],
        //J:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(2, 0)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(0, 0)]},
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(0, 2)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(2, 2)]},
        ],
        //L:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(0, 0)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(0, 2)]},
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(2, 2)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(2, 0)]},
        ],
        //S:
        vec![
            Brick{dots:[Dot(0, 0), Dot(1, 0), Dot(1, 1), Dot(2, 1)]},
            Brick{dots:[Dot(1, 2), Dot(1, 1), Dot(2, 1), Dot(2, 0)]},
        ],
        //Z:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(1, 0), Dot(2, 0)]},
            Brick{dots:[Dot(2, 2), Dot(2, 1), Dot(1, 1), Dot(1, 0)]},
        ],
        //T:
        vec![
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(1, 0)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(0, 1)]},
            Brick{dots:[Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(1, 2)]},
            Brick{dots:[Dot(1, 0), Dot(1, 1), Dot(1, 2), Dot(2, 1)]},
        ],
    ];
}
