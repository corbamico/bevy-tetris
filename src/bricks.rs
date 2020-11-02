#![allow(dead_code)]
use crate::consts::{BOARD_X, BOARD_X_Y, BOARD_Y, BOARD_Y_VALIDE, BRICKS_DICT, BRICKS_TYPES};
use rand::prelude::*;
#[derive(Copy, Clone, Default, Debug)]
pub struct Dot(pub i8, pub i8);
#[derive(Copy, Clone, Default)]
pub struct BrickOrgDot(Dot);

impl Dot {
    pub fn with_orignal_dot(&self, orig: &Dot) -> Self {
        Self(self.0 + orig.0, self.1 + orig.1)
    }
    pub fn move_left(&mut self) {
        self.0 -= 1;
    }
    pub fn move_right(&mut self) {
        self.0 += 1;
    }
    pub fn move_down(&mut self) {
        self.1 -= 1;
    }
    pub fn left(&self) -> Self {
        Self(self.0 - 1, self.1)
    }
    pub fn right(&self) -> Self {
        Self(self.0 + 1, self.1)
    }
    pub fn down(&self) -> Self {
        Self(self.0, self.1 - 1)
    }
}

#[derive(Copy, Clone)]
pub struct Brick {
    pub dots: [Dot; 4],
}

#[derive(Copy, Clone)]
pub struct BrickShape(pub usize, pub usize);

impl Into<Brick> for BrickShape {
    fn into(self) -> Brick {
        BRICKS_DICT[self.0][self.1]
    }
}

//BUG?
//&brickshape dont have same lifecycle of BRICKS_DICT
impl<'a> Into<&'a Brick> for &'a BrickShape {
    fn into(self) -> &'a Brick {
        &BRICKS_DICT[self.0][self.1]
    }
}

impl BrickShape {
    pub fn rand() -> Self {
        let index = rand::thread_rng().gen_range(0, BRICKS_TYPES);
        Self(index, 0)
        //Self(0, 0)
    }
    pub fn rotate(&self) -> Self {
        Self(self.0, (self.1 + 1) % BRICKS_DICT[self.0].len())
    }
}

#[derive(Debug)]
pub struct Board(Vec<bool>);

impl Default for Board {
    fn default() -> Self {
        Self(vec![false; BOARD_X_Y])
    }
}
impl Board {
    fn index(dot: &Dot) -> usize {
        (dot.0 as usize + dot.1 as usize * BOARD_X as usize) as usize
    }
    pub fn occupy_dot(&mut self, dot: &Dot) -> &mut Self {
        let i = Self::index(dot);
        if i < BOARD_X_Y {
            self.0[i] = true
        }
        self
    }
    pub fn occupy_brick(&mut self, brick: &Brick, orig: &Dot) {
        for i in 0..4 {
            self.occupy_dot(&brick.dots[i].with_orignal_dot(orig));
        }
    }

    pub fn occupy_brickshape(&mut self, brick_shape: &BrickShape, orig: &Dot) {
        self.occupy_brick(brick_shape.into(), orig)
    }

    pub fn occupied_dot(&self, dot: &Dot) -> bool {
        let i = Self::index(dot);
        if i < BOARD_X_Y {
            self.0[i]
        } else {
            false
        }
    }
    pub fn conflict_brick(&self, brick: &Brick, orig: &Dot) -> bool {
        self.occupied_dot(&brick.dots[0].with_orignal_dot(orig))
            || self.occupied_dot(&brick.dots[1].with_orignal_dot(orig))
            || self.occupied_dot(&brick.dots[2].with_orignal_dot(orig))
            || self.occupied_dot(&brick.dots[3].with_orignal_dot(orig))
    }
    fn dot_in_board(dot: &Dot) -> bool {
        //0 <= dot.0 && dot.0 < BOARD_X && 0 <= dot.1 && dot.1 < BOARD_Y
        //BUG: should we compare Y ?
        0 <= dot.0 && dot.0 < BOARD_X && 0 <= dot.1
    }
    fn brick_in_board(brick: &Brick, orig: &Dot) -> bool {
        Self::dot_in_board(&brick.dots[0].with_orignal_dot(orig))
            && Self::dot_in_board(&brick.dots[1].with_orignal_dot(orig))
            && Self::dot_in_board(&brick.dots[2].with_orignal_dot(orig))
            && Self::dot_in_board(&brick.dots[3].with_orignal_dot(orig))
    }
    pub fn valid_brick(&self, brick: &Brick, orig: &Dot) -> bool {
        Self::brick_in_board(brick, orig) && !self.conflict_brick(brick, orig)
    }
    pub fn valid_brickshape(&self, brick_shape: &BrickShape, orig: &Dot) -> bool {
        self.valid_brick(&(*brick_shape).into(), orig)
    }
    pub fn clear(&mut self) {
        for i in 0..BOARD_X_Y {
            self.0[i] = false
        }
    }
    pub fn can_clean(&self) -> bool {
        (0..BOARD_Y).any(|y| self.can_clean_line(y))
    }
    pub fn can_clean_line(&self, y: i8) -> bool {
        assert!(0 <= y);
        assert!(y < BOARD_Y);
        self.0[Self::index(&Dot(0, y))..Self::index(&Dot(0, y + 1))]
            .iter()
            .all(|x| *x)
    }
    pub fn get_clean_lines(&self) -> Vec<i8> {
        let mut vec = Vec::with_capacity(4);
        for i in (0..BOARD_Y).rev() {
            if self.can_clean_line(i) {
                vec.push(i);
            }
        }
        vec
    }
    pub fn clean_lines(&mut self) {
        let deleted_lines = self.get_clean_lines();
        for line in deleted_lines {
            self.clean_line(line);
        }
    }

    pub fn clean_line(&mut self, y: i8) {
        assert!(0 <= y);
        assert!(y < BOARD_Y);

        let dst_below = Self::index(&Dot(0, y));
        let src_below = Self::index(&Dot(0, y + 1));
        let src_high = Self::index(&Dot(0, BOARD_Y));

        //step 1.copy from tail
        self.0.copy_within(src_below..src_high, dst_below);
        //step 2.set last line as false
        self.0[Self::index(&Dot(0, BOARD_Y - 1))..Self::index(&Dot(0, BOARD_Y))]
            .iter_mut()
            .for_each(|x| *x = false);
    }
    pub fn game_over(&self) -> bool {
        self.0[Self::index(&Dot(0, BOARD_Y_VALIDE))..]
            .iter()
            .any(|x| *x)
    }
}
