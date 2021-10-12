use crate::models::board::Board;

pub struct PlayerState {
    buttons: u32,
    position: u32,
    board: Board,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            buttons: 5,
            position: 0,
            board: Board::new(),
        }
    }
}
