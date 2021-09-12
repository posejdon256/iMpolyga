use actix_cors::Cors;
use actix_web::{
    get, http, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use serde::Serialize;

mod actor;
mod communication;
mod context;
mod model;

use actor::WsActor;
use communication::endpoints;
use communication::websocket;
use context::{ContextAddr, ServerContext};

async fn index(
    server_context: web::Data<ContextAddr>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("lolz");
    let ctx: &ContextAddr = server_context.get_ref();
    //println!("{}", ctx.id);
    let resp = ws::start(WsActor {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ctx = ServerContext::new();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_origin("ws://localhost")
            .allowed_origin_fn(|_origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .data(ctx.clone())
            .wrap(cors)
            .route("/ws/", web::get().to(index))
            .service(endpoints::hello)
            .service(endpoints::echo)
            .service(endpoints::get_map)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
