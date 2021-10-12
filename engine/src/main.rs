use lazy_static::lazy_static;
use models::tile::Tile;
use crate::models::game_state::GameState;

mod models;

const LEATHER_TS: [u32; 5] = [20, 26, 32, 44, 50];
const BUTTON_TS: [u32; 8] = [5, 11, 17, 23, 29, 35, 41, 53];

lazy_static! {
    static ref TILES: [Tile; 34] = [
        Tile::new(2, 1, 0, "x x"),
        Tile::new(1, 3, 0, "ox xx"),
        Tile::new(3, 1, 0, "ox xx"),
        Tile::new(2, 2, 0, "x x x"),
        Tile::new(3, 2, 1, "ox xx xo"),
        Tile::new(2, 2, 0, "xo xx xx"),
        Tile::new(1, 4, 1, "ooxoo xxxxx ooxoo"),

        Tile::new(0, 3, 1, "oxo xxx oxo oxo"),
        Tile::new(6, 5, 2, "xx xx"),
        Tile::new(4, 2, 0, "xo xx xx ox"),
        Tile::new(2, 2, 0, "ox xx ox"),
        Tile::new(1, 5, 1, "xx ox ox xx"),
        Tile::new(3, 3, 1, "x x x x"),

        Tile::new(7, 1, 1, "xxxxx"),
        Tile::new(3, 4, 1, "xo xo xx xo"),
        Tile::new(7, 4, 2, "xo xx xx xo"),
        Tile::new(3, 6, 2, "oxo xxx xox"),
        Tile::new(2, 1, 0, "oxo oxx xxo oxo"),

        Tile::new(4, 6, 2, "ox ox xx"),
        Tile::new(5, 4, 2, "oxo xxx oxo"),
        Tile::new(2, 3, 0, "xox xxx xox"),
        Tile::new(5, 3, 1, "oxo xxx xxx oxo"),
        Tile::new(10, 3, 2, "ox ox ox xx"),
        Tile::new(5, 5, 2, "oxo oxo xxx"),

        Tile::new(10, 5, 3, "xx xx ox ox"),
        Tile::new(1, 2, 0, "xxo oxo oxo oxx"),
        Tile::new(4, 2, 1, "xo xo xx"),
        Tile::new(7, 2, 2, "oxo oxo oxo xxx"),
        Tile::new(10, 4, 3, "xoo xxo oxx"),
        Tile::new(1, 2, 0, "xox xxx"),

        Tile::new(2, 3, 1, "ox ox xx xo"),
        Tile::new(7, 6, 3, "oxx xxo"),
        Tile::new(8, 6, 3, "oxx oxx xxo"),

        Tile::new(0, 0, 0, "x"), // leather
    ];
}

fn main() {
    println!("Hello, world!");
    for tile in TILES.iter() {
        tile.display()
    }

    let game_state = GameState::new();

    println!("{:?}", game_state.tiles_order);
    println!("{:?}", game_state.get_next_tile_ids());
}
