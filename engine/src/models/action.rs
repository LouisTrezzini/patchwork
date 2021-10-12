use crate::models::tile_location::TileLocation;

pub enum Action {
    Advance,
    PlaceTile { tile_id: u32, location: TileLocation }
}
