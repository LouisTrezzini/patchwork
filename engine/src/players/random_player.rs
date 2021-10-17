use rand::Rng;
use crate::models::action::Action;
use crate::models::color::Color;
use crate::models::game_state::GameState;
use crate::models::player::Player;
use crate::models::tile_location::{Orientation, TileLocation};

pub struct RandomPlayer {
    color: Color,
}

impl RandomPlayer {
    pub fn new(color: Color) -> RandomPlayer {
        RandomPlayer {
            color
        }
    }
}

impl Player for RandomPlayer {
    fn get_action(&self, game_state: &GameState) -> Action {
        let available_tiles = game_state.get_available_tile_ids_for(self.color);

        // TODO sort by expected value
        let player_state = match self.color {
            Color::Green => &game_state.green_state,
            Color::Yellow => &game_state.yellow_state,
        };

        for i in 0..available_tiles.len() {
            for r in 0..8 {
                for c in 0..8 {
                    let tile_location = TileLocation::new(
                        available_tiles[i],
                        r,
                        c,
                        Orientation::Base,
                    );
                    if player_state.board.can_place_tile(&tile_location) {
                        return Action::PlaceTile(tile_location);
                    }
                }
            }
        }

        Action::Advance
    }
}
