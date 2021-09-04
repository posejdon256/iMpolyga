use actix_web::{get, post, http, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use actix_cors::Cors;
use serde::{Serialize};

#[derive(Serialize)]
struct Tile {
    color: String,
}
#[derive(Serialize)]
struct Map {
    id: usize,
    size: usize,
    tiles: Vec<Vec<Tile>>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/map")]
async fn getMap() -> actix_web::Result<web::Json<Map>> {
    let mut map = Map{id: 12, size: 0, tiles: vec![
        vec![Tile{color: "white".to_owned()}, Tile{color: "black".to_owned()}],
        vec![Tile{color: "red".to_owned()}, Tile{color: "blue".to_owned()}],
    ]};
    Ok(web::Json(map))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

struct WsActor;

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsActor {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
              .allowed_origin("http://localhost")
              .allowed_origin_fn(|origin, _req_head| {
                  true
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
            .wrap(cors)
            .route("/ws/", web::get().to(index))
            .service(hello)
            .service(echo)
            .service(getMap)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}