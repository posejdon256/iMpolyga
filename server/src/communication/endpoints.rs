use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use crate::model::map::{Map, Tile};


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/map")]
pub async fn getMap() -> actix_web::Result<web::Json<Map>> {
    let mut map = Map{id: 12, size: 0, tiles: vec![
        vec![Tile{color: "green".to_owned()}, Tile{color: "black".to_owned()}],
        vec![Tile{color: "red".to_owned()}, Tile{color: "blue".to_owned()}],
    ]};
    Ok(web::Json(map))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}