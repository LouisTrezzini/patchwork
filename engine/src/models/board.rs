use crate::models::tile_location::TileLocation;

pub struct Board {
    tile_locations: Vec<TileLocation>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tile_locations: Vec::new(),
        }
    }
}
