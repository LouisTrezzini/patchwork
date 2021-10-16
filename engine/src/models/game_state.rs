use std::cmp::min;
use std::convert::TryFrom;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::models::action::Action;
use crate::models::color::Color;
use crate::models::color::Color::Green;
use crate::models::constants::{BUTTON_TS, NUM_TS};
use crate::models::player_state::PlayerState;
use crate::models::tile::TILES;
use crate::models::tile_location::TileLocation;

struct PlayerAction(Color, Action);

pub struct GameState {
    tiles_order: [usize; 33],
    starting_player: Color,
    actions: Vec<PlayerAction>, // player could be deducted

    // derived state
    current_player: Option<Color>,
    used_tiles: [bool; 33],
    pub yellow_state: PlayerState,
    pub green_state: PlayerState,
}

impl GameState {
    pub fn new() -> GameState {
        let mut rng = rand::thread_rng();
        let mut tile_ids: Vec<usize> = (0..33).collect();
        tile_ids.shuffle(&mut rng);

        let starting_player = match rng.gen() {
            true => Color::Green,
            false => Color::Yellow
        };

        GameState {
            tiles_order: <[usize; 33]>::try_from(tile_ids.as_slice()).unwrap(),
            starting_player,
            actions: Vec::new(),
            used_tiles: [false; 33],
            current_player: Some(starting_player),
            yellow_state: PlayerState::new(),
            green_state: PlayerState::new(),
        }
    }

    fn get_next_tile_ids(&self) -> [usize; 3] {
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

    pub fn get_available_tile_ids_for(&self, player: Color) -> Vec<usize> {
        let mut rv = Vec::new();

        for tile_id in self.get_next_tile_ids() {
            if player == Color::Green && self.green_state.buttons >= TILES[tile_id].button_cost() {
                rv.push(tile_id);
            }
            if player == Color::Yellow && self.yellow_state.buttons >= TILES[tile_id].button_cost() {
                rv.push(tile_id);
            }
        }

        rv
    }

    pub fn get_current_player(&self) -> Option<Color> {
        return self.current_player;
    }

    fn update_current_player(&mut self) {
        if self.yellow_state.position == NUM_TS && self.green_state.position == NUM_TS {
            self.current_player = None;
        }
        if self.yellow_state.position < self.green_state.position {
            self.current_player = Some(Color::Yellow);
        } else if self.green_state.position < self.yellow_state.position {
            self.current_player = Some(Color::Green);
        } else {
            // don't update current player is "on top" and will play again
        }
    }

    pub fn apply_action(&mut self, color: Color, action: Action) {
        if color != self.current_player.unwrap() {
            panic!("illegal state");
        }

        match action {
            Action::Advance => {
                self.advance_player(color);
            }
            Action::PlaceTile(tile_location) => {
                self.place_tile(color, &tile_location);
            }
        }

        self.update_current_player();
    }

    fn advance_player(&mut self, color: Color) {
        let (player_state, opponent_state) = GameState::extract_state(color, &mut self.green_state, &mut self.yellow_state);

        let prev_position = player_state.position;
        player_state.position = min(opponent_state.position + 1, NUM_TS);
        let diff = player_state.position - prev_position;
        if diff == 0 {
            panic!("illegal state");
        }
        player_state.buttons += diff as u32;
        GameState::apply_events(player_state, prev_position);
    }

    fn place_tile(&mut self, color: Color, tile_location: &TileLocation) {
        let available_tile_ids = self.get_available_tile_ids_for(color);
        if !available_tile_ids.contains(&tile_location.tile_id()) {
            panic!("illegal state");
        }
        self.used_tiles[tile_location.tile_id()] = true;
        {
            let (player_state, _) = GameState::extract_state(color, &mut self.green_state, &mut self.yellow_state);
            player_state.buttons -= TILES[tile_location.tile_id()].button_cost();
            // TODO add to board
            player_state.position += TILES[tile_location.tile_id()].time_cost();
        }
    }

    fn apply_events(player_state: &mut PlayerState, prev_position: usize) {
        for i in (prev_position + 1)..(player_state.position + 1) {
            if BUTTON_TS.contains(&i) {
                player_state.buttons += player_state.board.income()
            }
            // TODO leather
        }
    }

    fn extract_state<'a>(color: Color<>, green_state: &'a mut PlayerState, yellow_state: &'a mut PlayerState) -> (&'a mut PlayerState, &'a mut PlayerState) {
        match color {
            Color::Green => (green_state, yellow_state),
            Color::Yellow => (yellow_state, green_state),
        }
    }
}
