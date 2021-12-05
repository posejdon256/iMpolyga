// use actix_cors::Cors;
// use actix_web::{
//     get, http, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
// };
// use actix_web_actors::ws;
// use serde::Serialize;

//mod actor;
mod communication;
//mod context;
mod client_node;
mod connection_coordinator;
mod model;
mod user_connection;

use communication::websocket::ws_entry;
use model::map::*;

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /chat -> websocket upgrade
    let connector = ws_entry("ws");
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

    let routes = index.or(get_map).or(connector).with(cors);

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
