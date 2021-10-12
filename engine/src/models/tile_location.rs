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

pub struct TileLocation {
    tile_id: char,
    x: u32,
    y: u32,
    orientation: Orientation,
}
