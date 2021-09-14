use crate::communication::messages::{IncomingMessage, OutgoingMessage};
use crate::model::map::{Map, Tile, TileType};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use serde::Serialize;
use serde_json;

pub struct WsActor;

impl WsActor {
    fn parse_incoming_message(content: &str) -> Result<IncomingMessage, ()> {
        let parsed = serde_json::from_str::<IncomingMessage>(content);
        match parsed {
            Ok(x) => Ok(x),
            Err(_) => Err(()),
        }
    }

    fn handle_message(&mut self, msg: IncomingMessage, ctx: &mut <Self as Actor>::Context) {
        match msg {
            IncomingMessage::GetMap => {
                self.send_map(ctx);
            }
            IncomingMessage::MapById(id) => {
                self.send_map(ctx);
            }
            IncomingMessage::RandomMap(seed) => {
                self.send_map(ctx);
            }
        }
    }
    fn send_map(&mut self, ctx: &mut <Self as Actor>::Context) {
        let map = Map {
            id: 12,
            size: 0,
            tiles: vec![
                vec![
                    Tile {
                        id: TileType::Mountain,
                    },
                    Tile {
                        id: TileType::Grass,
                    },
                ],
                vec![
                    Tile {
                        id: TileType::Ocean,
                    },
                    Tile {
                        id: TileType::Mountain,
                    },
                ],
            ],
        };
        ctx.text(serde_json::to_string(&OutgoingMessage::Map(map)).unwrap());
    }
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("{}", text);
                let parsed_message = Self::parse_incoming_message(&text);
                match parsed_message {
                    Ok(msg) => self.handle_message(msg, ctx),
                    Err(_) => ctx.ping("eyy".as_bytes()),
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
