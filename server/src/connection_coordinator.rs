use futures_util::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::client_node::{create_client_node, ClientNodeChannel, ClientNodeMessage};
use crate::user_connection::{user_connection_task, ConnectionData};

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<warp::ws::Message>>>>;

pub enum CoordinatorMessage {
    NewConnection(ConnectionData),
}

struct ConnectionCoordinator {
    receiver: UnboundedReceiverStream<CoordinatorMessage>,
    self_sender: CoordinatorChannel,
    users: Users,
    client_node: ClientNodeChannel,
}

impl ConnectionCoordinator {
    fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
        let rx = UnboundedReceiverStream::new(rx);
        let client_node = create_client_node(tx.clone());
        ConnectionCoordinator {
            receiver: rx,
            self_sender: tx,
            users: Users::default(),
            client_node,
        }
    }

    fn run(mut self) {
        tokio::task::spawn(async move {
            while let Some(message) = self.receiver.next().await {
                match message {
                    CoordinatorMessage::NewConnection(connection_data) => {
                        eprintln!(
                            "Coordinator: new user connected: {}",
                            connection_data.connection_id
                        );
                        // assign client node and pass conection to client node instead
                        let users = self.users.clone();
                        tokio::task::spawn(async move {
                            user_connection_task(connection_data).await;
                        });
                        self.client_node
                            .send(ClientNodeMessage::AcceptUser(connection_data));
                    }
                }
            }
        });
    }
}

pub type CoordinatorChannel = mpsc::UnboundedSender<CoordinatorMessage>;

pub fn create_coordinator() -> CoordinatorChannel {
    let coordinator = ConnectionCoordinator::new();
    let tx = coordinator.self_sender.clone();
    coordinator.run();
    tx
}
