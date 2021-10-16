use crate::models::tile_location::TileLocation;

#[derive(Debug)]
pub enum Action {
    Advance,
    PlaceTile(TileLocation)
}
