// Websocket initialization
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

struct ConnectionData {
    messageSink: mpsc::UnboundedSender<warp::ws::Message>,
    messageProducer: futures_util::stream::SplitStream<warp::ws::WebSocket>,
}
enum CoordinatorMessage {
    NewConnection(ConnectionData),
}

enum ConnectionState {
    WaitingForClientNodeAssignment,
}

struct ConnectionCoordinator {
    receiver: UnboundedReceiverStream<CoordinatorMessage>,
    self_sender: CoordinatorChannel,
    users: Users,
}

impl ConnectionCoordinator {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
        let rx = UnboundedReceiverStream::new(rx);
        ConnectionCoordinator {
            receiver: rx,
            self_sender: tx,
            users: Users::default(),
        }
    }

    pub fn run(mut self) {
        tokio::task::spawn(async move {
            while let Some(message) = self.receiver.next().await {
                match message {
                    CoordinatorMessage::NewConnection(connection_data) => {
                        // Use a counter to assign a new unique ID for this user.
                        let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

                        eprintln!("Coordinator: new user connected: {}", my_id);
                        // assign client node and pass conection to client node instead
                        let users = self.users.clone();
                        tokio::task::spawn(async move {
                            user_connection_task(
                                users,
                                my_id,
                                connection_data.messageSink,
                                connection_data.messageProducer,
                            )
                            .await;
                        });
                    }
                }
            }
        });
    }
}

type CoordinatorChannel = mpsc::UnboundedSender<CoordinatorMessage>;

fn create_coordinator() -> CoordinatorChannel {
    let coordinator = ConnectionCoordinator::new();
    let tx = coordinator.self_sender.clone();
    coordinator.run();
    tx
}

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

async fn user_connection_task(
    users: Users,
    my_id: usize,
    tx: mpsc::UnboundedSender<warp::ws::Message>,
    user_ws_rx: futures_util::stream::SplitStream<warp::ws::WebSocket>,
) {
    let mut user_ws_rx = user_ws_rx;
    // Save the sender in our list of connected users.
    users.write().await.insert(my_id, tx);

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        user_message(my_id, msg, &users).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users).await;
}
async fn pass_user_to_coordinator(ws: WebSocket, coordinator: CoordinatorChannel) {
    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let user_data = ConnectionData {
        messageSink: tx,
        messageProducer: user_ws_rx,
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

async fn user_message(my_id: usize, msg: Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in users.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn user_disconnected(my_id: usize, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}
