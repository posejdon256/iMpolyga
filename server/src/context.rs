use actix::{Actor, Addr, Context};

use crate::actor::WsActor;

pub struct ServerContext {
    pub id: usize,
    players: Vec<Addr<WsActor>>,
}
pub type ContextAddr = Addr<ServerContext>;

impl ServerContext {
    pub fn new() -> ContextAddr {
        ServerContext {
            id: 1,
            players: vec![],
        }
        .start()
    }
}

impl Actor for ServerContext {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {}
}
