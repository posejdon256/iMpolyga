use crate::model::map::{Map, Tile, TileType};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/map")]
pub async fn get_map() -> actix_web::Result<web::Json<Map>> {
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
    Ok(web::Json(map))
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
