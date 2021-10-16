use array2d::Array2D;
use lazy_static::lazy_static;

use crate::models::tile_location::TileLocation;
use crate::models::tile::TILES;

pub struct Board {
    tile_locations: Vec<TileLocation>,
    mask: Array2D<bool>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tile_locations: Vec::new(),
            mask: Array2D::filled_with(false, 9, 9),
        }
    }

    pub fn income(&self) -> u32 {
        self.tile_locations.iter().map(|x| TILES[x.tile_id()].button_income()).sum()
    }

    pub fn count_empty_squares(&self) -> u32 {
        let mut count = 0;

        for elem in self.mask.elements_column_major_iter() {
            if !elem {
                count += 1
            }
        }
        count
    }
}
