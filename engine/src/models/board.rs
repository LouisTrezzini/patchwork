use array2d::Array2D;
use lazy_static::lazy_static;
use crate::models::constants::TILES_CHAR;

use crate::models::tile::TILES;
use crate::models::tile_location::TileLocation;

pub struct Board {
    tile_locations: Vec<TileLocation>,
    mask: Array2D<Option<usize>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tile_locations: Vec::new(),
            mask: Array2D::filled_with(None, 9, 9),
        }
    }

    pub fn add_tile(&mut self, tile_location: &TileLocation) {
        if !self.can_place_tile(tile_location) {
            panic!("illegal state");
        }

        self.tile_locations.push(*tile_location);
        for (r, c) in tile_location.get_mask() {
            self.mask.set(r, c, Some(tile_location.tile_id()));
        }
    }

    pub fn can_place_tile(&self, tile_location: &TileLocation) -> bool {
        for (r, c) in tile_location.get_mask() {
            let value = self.mask.get(r, c);
            if value.is_none() { // out of bounds
                return false
            }

            if !value.unwrap().is_none() {
                return false;
            }
        }

        true
    }

    pub fn income(&self) -> u32 {
        self.tile_locations.iter().map(|x| TILES[x.tile_id()].button_income()).sum()
    }

    pub fn count_empty_squares(&self) -> u32 {
        let mut count = 0;

        for elem in self.mask.elements_column_major_iter() {
            if elem.is_none() {
                count += 1
            }
        }
        count
    }

    pub fn display(&self) {
        for row in self.mask.rows_iter() {
            for cell in row {
                match cell {
                    Some(tile_id) => print!("{}", TILES_CHAR[*tile_id]),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}
