// Websocket initialization
use crate::connection_coordinator::{create_coordinator, CoordinatorChannel, CoordinatorMessage};
use crate::user_connection::ConnectionData;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;
use warp::Filter;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

pub fn ws_entry(
    path: &str,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // Turn our "state" into a new Filter...
    let coordinator = create_coordinator();
    let coordinator_filter = warp::any().map(move || coordinator.clone());
    warp::path(path.to_owned())
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(coordinator_filter)
        .map(|ws: warp::ws::Ws, coordinator| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| pass_user_to_coordinator(socket, coordinator))
        })
}

async fn pass_user_to_coordinator(ws: WebSocket, coordinator: CoordinatorChannel) {
    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let connection_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    let user_data = ConnectionData {
        messageSink: tx,
        messageProducer: user_ws_rx,
        connection_id,
    };

    match coordinator.send(CoordinatorMessage::NewConnection(user_data)) {
        Ok(_) => {
            // connection passed to coordinator, spawn relay task
            // In other words: task sending messages to user
            tokio::task::spawn(async move {
                // it will close itself once websocket connection is dropped
                while let Some(message) = rx.next().await {
                    user_ws_tx
                        .send(message)
                        .unwrap_or_else(|e| {
                            eprintln!("websocket send error: {}", e);
                        })
                        .await;
                }
            });
        }
        Err(_error) => {
            // coordinator channel problem -> inform user about temporarily unavailable service and
            // either drop connection or retry(?)
        }
    }
}
