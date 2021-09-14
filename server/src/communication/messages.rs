use serde::{Deserialize, Serialize};
use crate::model::map::Map;


#[derive(Serialize, Deserialize)]
//#[serde(tag = "type")]
pub enum IncomingMessage{
    GetMap,
    RandomMap(u32),
    MapById(u32)
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OutgoingMessage {
    Map(Map),
}