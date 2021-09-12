use serde::{Serialize};

#[derive(Serialize)]
pub struct Tile {
    pub color: String,
}
#[derive(Serialize)]
pub struct Map {
    pub id: usize,
    pub size: usize,
    pub tiles: Vec<Vec<Tile>>,
}