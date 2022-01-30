use piraffe_common::{MessageToCoordinator, MessageFromCoordinator, make_welcome};
use crossbeam::channel::{Receiver, Sender};
use flatbuffers::FlatBufferBuilder;

pub struct Coordinator {
    to_clients: Sender<MessageFromCoordinator>,
    from_clients: Receiver<MessageToCoordinator>,
}

impl Coordinator {
    pub fn new(to_clients: Sender<MessageFromCoordinator>,
           from_clients: Receiver<MessageToCoordinator>) -> Self {
        Coordinator { to_clients: to_clients, from_clients: from_clients}
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(x) = self.from_clients.recv() {
                match x {
                    MessageToCoordinator::Connect(connect) => {
                        log::info!("Client connected, {}", connect.name);
                        let mut builder = FlatBufferBuilder::new();
                        let mut buffer: Vec<u8> = Vec::new();
                        make_welcome(&mut builder, &mut buffer, "cool server");
                        self.to_clients.send(MessageFromCoordinator{topic: None, message_data: buffer }).expect("failed to send");
                    }
                    _ => {
                        log::warn!("Unsupported message :(")
                    }
                }
            }
        }
    }
}