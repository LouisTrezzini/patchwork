use crate::models::action::Action;
use crate::models::game_state::GameState;

pub trait Player {
    fn get_action(&self, game_state: &GameState) -> Action;
}
