use serde::{Serialize};
use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum TileType {
    Ocean = 0,
    Mountain = 1,
    Grass = 2,
}
#[derive(Serialize)]
pub struct Tile {
    pub id: TileType,
}
#[derive(Serialize)]
pub struct Map {
    pub id: usize,
    pub size: usize,
    pub tiles: Vec<Vec<Tile>>,
}