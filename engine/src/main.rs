use crate::models::game_state::GameState;
use crate::models::color::Color;
use crate::models::game::Game;
use crate::models::tile::TILES;
use crate::players::random_player::RandomPlayer;

mod models;
mod players;

fn main() {
    println!("Hello, world!");
    for tile in TILES.iter() {
        tile.display()
    }

    let yellow_player = RandomPlayer::new(Color::Yellow);
    let green_player = RandomPlayer::new(Color::Green);
    let mut game = Game::new(Box::new(yellow_player), Box::new(green_player));

    game.play_game();
}
