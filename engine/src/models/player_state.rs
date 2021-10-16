use std::fmt::{Display, Formatter};
use crate::models::board::Board;

pub struct PlayerState {
    pub buttons: u32,
    pub position: usize,
    pub board: Board,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            buttons: 5,
            position: 0,
            board: Board::new(),
        }
    }

    pub fn score(&self) -> i32 {
        // TODO special 7x7 tile
        let special_score = 0;
        (self.buttons + special_score) as i32 - 2 * self.board.count_empty_squares() as i32
    }
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[buttons: {}, position: {}]", self.buttons, self.position)
    }
}
