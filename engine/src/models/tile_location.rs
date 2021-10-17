use crate::models::tile::TILES;

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum Orientation {
    Base,
    Base90,
    Base180,
    Base270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct TileLocation {
    tile_id: usize,
    r: usize,
    c: usize,
    orientation: Orientation,
}

impl TileLocation {
    pub fn new(
        tile_id: usize,
        r: usize,
        c: usize,
        orientation: Orientation,
    ) -> TileLocation {
        TileLocation {
            tile_id,
            r,
            c,
            orientation,
        }
    }

    pub fn tile_id(&self) -> usize {
        self.tile_id
    }
    pub fn r(&self) -> usize {
        self.r
    }
    pub fn c(&self) -> usize {
        self.c
    }
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn get_mask(&self) -> Vec<(usize, usize)> {
        let mut rv: Vec<(usize, usize)> = Vec::new();

        let tile = &TILES[self.tile_id];

        for r in 0..tile.shape().num_rows() {
            for c in 0..tile.shape().num_columns() {
                match tile.shape().get(r, c) {
                    Some(true) => {
                        rv.push((self.r + r, self.c + c)); // TODO orientation
                    },
                    Some(false) => {},
                    None => panic!("illegal state")
                }
            }
        }

        rv
    }
}
