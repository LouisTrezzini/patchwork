#[derive(Debug)]
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

#[derive(Debug)]
pub struct TileLocation {
    tile_id: usize,
    x: u32,
    y: u32,
    orientation: Orientation,
}

impl TileLocation {
    pub fn new(
        tile_id: usize,
        x: u32,
        y: u32,
        orientation: Orientation,
    ) -> TileLocation {
        TileLocation {
            tile_id,
            x,
            y,
            orientation,
        }
    }

    pub fn tile_id(&self) -> usize {
        self.tile_id
    }
    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }
}
