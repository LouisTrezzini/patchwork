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
        println!("{:?} {:?}", available_tiles, self.color);
        if available_tiles.len() == 0 {
            return Action::Advance;
        }

        let mut rng = rand::thread_rng();

        let tile_id = available_tiles[rng.gen_range(0..available_tiles.len())];

        Action::PlaceTile(
            TileLocation::new(
                tile_id,
                0,
                0,
                Orientation::Base,
            )
        )
    }
}
