use std::convert::TryFrom;
use crate::models::action::Action;
use crate::models::player::Player;
use crate::models::player_state::PlayerState;

use rand::seq::SliceRandom;

struct PlayerAction(Player, Action);

pub struct GameState {
    pub tiles_order: [usize; 33],
    actions: Vec<PlayerAction>, // player could be deducted

    // derived state
    used_tiles: [bool; 33],
    yellow_state: PlayerState,
    green_state: PlayerState,
}

impl GameState {
    pub fn new() -> GameState {
        let mut rng = rand::thread_rng();
        let mut tile_ids: Vec<usize> = (0..33).collect();
        tile_ids.shuffle(&mut rng);

        GameState{
            tiles_order: <[usize; 33]>::try_from(tile_ids.as_slice()).unwrap(),
            actions: Vec::new(),
            used_tiles: [false; 33],
            yellow_state: PlayerState::new(),
            green_state: PlayerState::new()
        }
    }

    pub fn get_next_tile_ids(&self) -> [usize; 3] {
        let mut rv = [0; 3];
        let mut found = 0;
        for tile_id in self.tiles_order {
            if !self.used_tiles[tile_id] {
                rv[found] = tile_id;
                found += 1;
            }
            if found >= 3 {
                break;
            }
        }
        rv
    }
}
