use std::io::{Read, Write};
use std::net::TcpStream;
use crossbeam::channel::{Receiver, Sender};
use piraffe_common::fb_generated::{All, root_as_proto};
use piraffe_common::{MessageFromCoordinator, MessageToCoordinator, ConnectionMessage};

pub struct ClientListener {
    to_coordinator: Sender<MessageToCoordinator>,
    from_coordinator: Receiver<MessageFromCoordinator>,
    stream: TcpStream,
    buf: [u8; 7000],
    subscriptions: Vec<String>
}

impl ClientListener {
    pub fn new(from_coordinator: Receiver<MessageFromCoordinator>,
           to_coordinator: Sender<MessageToCoordinator>,
           stream: TcpStream) -> Self {
               ClientListener {
                   to_coordinator,
                   from_coordinator,
                   stream,
                   buf: [0u8; 7000],
                   subscriptions: Vec::new(),
               }

    }

    fn check_client_socket(&mut self) {

        match self.stream.read(&mut self.buf) {
            Ok(bytes) if bytes > 0 => {
                log::debug!("Received from server");
                match root_as_proto(&self.buf) {
                    Ok(msg) => {
                        println!("Got proto {} {:?}", bytes, msg);
                        match msg.message_type() {
                            All::Connection => {
                                let conn_msg = msg.message_as_connection();
                                let name = conn_msg.unwrap().name().unwrap();
                                self.to_coordinator.send(MessageToCoordinator::Connect(ConnectionMessage { name: String::from(name) }));
                            },
                            All::Subscribe => {
                                let sub_msg = msg.message_as_subscribe();
                                let topic = sub_msg.unwrap().topic().unwrap();
                                self.subscriptions.push(topic.to_string());
                            }
                            _ => {}
                        }
                    },
                    Err(err) => {eprintln!("Error: {}", err)}
                }
            }
            _ => {}

            
        }
    }

    fn check_coordinator_channel(&mut self) {
        if let Ok(msg) = self.from_coordinator.try_recv() {
            match msg.topic {
                Some(topic) => {
                    if self.subscriptions.contains(&topic)  {
                        self.stream.write_all(&msg.message_data).expect("failed to write to socket");
                    }
                },
                None => self.stream.write_all(&msg.message_data).expect("failed to write to socket"),
            }           

            
        }
    }

    pub fn run(&mut self) {
        

        self.stream.set_nonblocking(true).expect("failed to set nonblocking");

        loop {
            self.check_client_socket();
            self.check_coordinator_channel();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

}