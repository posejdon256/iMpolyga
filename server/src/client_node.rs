use crate::connection_coordinator::CoordinatorChannel;
use crate::user_connection::ConnectionData;
use futures_util::StreamExt;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub enum ClientNodeMessage {
    AcceptUser(ConnectionData),
    ReportAvailability,
}

pub type ClientNodeChannel = mpsc::UnboundedSender<ClientNodeMessage>;

struct ClientNode {
    receiver: UnboundedReceiverStream<ClientNodeMessage>,
    self_sender: ClientNodeChannel,
    users: HashMap<usize, ConnectionData>,
    coordinator: CoordinatorChannel,
}

impl ClientNode {
    fn new(coordinator: CoordinatorChannel) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<ClientNodeMessage>();
        let rx = UnboundedReceiverStream::new(rx);
        ClientNode {
            receiver: rx,
            self_sender: tx,
            users: Default::default(),
            coordinator,
        }
    }

    fn run(mut self) {
        tokio::task::spawn(async move {
            while let Some(message) = self.receiver.next().await {
                //
            }
        });
    }
}

pub fn create_client_node(coordinator: CoordinatorChannel) -> ClientNodeChannel {
    let client_node = ClientNode::new(coordinator);
    let tx = client_node.self_sender.clone();
    client_node.run();
    tx
}
