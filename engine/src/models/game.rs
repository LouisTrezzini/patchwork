use std::convert::TryFrom;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::models::action::Action;
use crate::models::game_state::GameState;
use crate::models::color::Color;
use crate::models::player::Player;
use crate::models::player_state::PlayerState;
use crate::models::tile::TILES;

pub struct Game {
    game_state: GameState,
    yellow_player: Box<dyn Player>,
    green_player: Box<dyn Player>,
}

impl Game {
    pub fn new(yellow_player: Box<dyn Player>,
               green_player: Box<dyn Player>) -> Game {
        Game {
            game_state: GameState::new(),
            yellow_player,
            green_player,
        }
    }

    pub fn play_game(&mut self) {
        while let Some(color) = self.game_state.get_current_player() {
            println!("==========");
            println!("available tiles: {:?}", self.game_state.get_available_tile_ids_for(color));
            println!("cost: {:?}", self.game_state.get_available_tile_ids_for(color).iter().map(|&id|TILES[id].button_cost()).collect::<Vec<u32>>());
            match color {
                Color::Green => self.game_state.green_state.board.display(),
                Color::Yellow => self.game_state.yellow_state.board.display(),
            };
            println!("asking {:?} for action", color);
            let action = match color {
                Color::Green => self.green_player.get_action(&self.game_state),
                Color::Yellow => self.yellow_player.get_action(&self.game_state),
            };
            println!("{:?} picked {:?}", color, action);
            self.game_state.apply_action(color, action);
            println!("{:?} is {}", color, match color {
                Color::Green => &self.game_state.green_state,
                Color::Yellow => &self.game_state.yellow_state,
            });
            match color {
                Color::Green => self.game_state.green_state.board.display(),
                Color::Yellow => self.game_state.yellow_state.board.display(),
            };
        }
        println!("game done. yellow score: {} green score: {}", self.game_state.yellow_state.score(), self.game_state.green_state.score());
    }
}
