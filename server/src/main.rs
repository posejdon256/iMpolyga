// use actix_cors::Cors;
// use actix_web::{
//     get, http, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
// };
// use actix_web_actors::ws;
// use serde::Serialize;

//mod actor;
mod communication;
//mod context;
mod model;

use model::map::*;
// use actor::WsActor;
// use communication::endpoints;
use communication::websocket::ws_entry;
// use context::{ContextAddr, ServerContext};

// async fn index(
//     server_context: web::Data<ContextAddr>,
//     req: HttpRequest,
//     stream: web::Payload,
// ) -> Result<HttpResponse, Error> {
//     println!("lolz");
//     let ctx: &ContextAddr = server_context.get_ref();
//     //println!("{}", ctx.id);
//     let resp = ws::start(WsActor {}, &req, stream);
//     println!("{:?}", resp);
//     resp
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let ctx = ServerContext::new();
//     HttpServer::new(move || {
//         let cors = Cors::default()
//             .allowed_origin("http://localhost")
//             .allowed_origin("ws://localhost")
//             .allowed_origin_fn(|_origin, _req_head| true)
//             .allowed_methods(vec!["GET", "POST"])
//             .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//             .allowed_header(http::header::CONTENT_TYPE)
//             .max_age(3600);
//         App::new()
//             .data(ctx.clone())
//             .wrap(cors)
//             .route("/ws/", web::get().to(index))
//             .service(endpoints::hello)
//             .service(endpoints::echo)
//             .service(endpoints::get_map)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// #![deny(warnings)]
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /chat -> websocket upgrade
    let chat = ws_entry();
    let get_map = warp::path("map").map(|| {
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
        warp::reply::json(&map)
    });

    let cors = warp::cors().allow_any_origin().build();
    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(get_map).or(chat).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>Warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        const chat = document.getElementById('chat');
        const text = document.getElementById('text');
        const uri = 'ws://' + location.host + '/ws';
        const ws = new WebSocket(uri);

        function message(data) {
            const line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }

        ws.onopen = function() {
            chat.innerHTML = '<p><em>Connected!</em></p>';
        };

        ws.onmessage = function(msg) {
            message(msg.data);
        };

        ws.onclose = function() {
            chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };

        send.onclick = function() {
            const msg = text.value;
            ws.send(msg);
            text.value = '';

            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;
