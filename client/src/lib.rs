use std::{net::TcpStream, slice::Iter};
use std::io::{Read, Write};

use messages::{Message, PublishMessage};
use piraffe_common::{make_connect, make_publish, make_subscribe, get_proto_message};
use piraffe_common::fb_generated::All;
use flatbuffers::*;

use crossbeam::channel::{Receiver, Sender, bounded};

mod messages;


pub struct PiraffeClient {
    server_uri: String,
    client_name: String,
    handle: std::thread::JoinHandle<()>,
    publish_channel: crossbeam::channel::Sender<messages::Message>,
}

fn send_encoded(tcp_tream: &mut TcpStream, msg: &messages::Message, builder: &mut FlatBufferBuilder) 
{
    match msg {
        messages::Message::Publish(pubmessage) => {
            // TODO: Reuse big vector?
            let mut vec: Vec<u8> = Vec::new();
            make_publish(builder, &mut vec,  &pubmessage.subject, &pubmessage.data);
            println!("Made buffer with {} B", vec.len());
            tcp_tream.write(&vec).expect("failed to send message");
        },
        messages::Message::Subscribe(submessage) => {
            let mut vec: Vec<u8> = Vec::new();
            make_subscribe(builder, &mut vec, &submessage.subject);
            tcp_tream.write(&vec).expect("Failed to send subscribe");
        }
        _ => {
            
        }
    }
}

impl PiraffeClient {
    pub fn new(server_uri: &str, client_name: &str) -> Self {

        let (sender, receiver): (Sender<Message>, Receiver<Message>) = bounded(10);

        
        let handle = std::thread::spawn({
            let s_uri = String::from(server_uri);
            let c_name = String::from(client_name);
            
            move || {
                let mut buffer = [0u8; 8000];
                let mut builder=  FlatBufferBuilder::new();
                let mut tcp_stream_res=  PiraffeClient::_connect(&s_uri);
                while tcp_stream_res.is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    tcp_stream_res = PiraffeClient::_connect(&s_uri);
                }

                let mut tcp_stream = tcp_stream_res.unwrap();

                let mut bytes: Vec<u8> = Vec::new();
                make_connect(&mut builder, &mut bytes, &c_name);
                println!("Made buffer with {} B", bytes.len());
                tcp_stream.write(&bytes).expect("failed to write");

                let mut sender: Option<Sender<messages::DataMessage>> = None;
                loop {
                    if let Ok(msg) = receiver.try_recv() {
                        send_encoded(&mut tcp_stream, &msg, &mut builder);

                        match msg {
                            messages::Message::Subscribe(submessage) => {
                                sender = Some(submessage.sender);
                            }
                            _ => {}
                        };
                    }

                    let (_, msg) = PiraffeClient::check_socket(&mut tcp_stream, &mut buffer, 0);

                    if let Some(msg) = msg {
                        match &sender {
                            Some(sender) => {sender.send(msg).expect("failed to send");},
                            None => {}
                        }
                    }

                };
            }

        });

        PiraffeClient {
            server_uri: server_uri.to_string(),
            client_name: client_name.to_string(),
            handle: handle,
            publish_channel: sender
        }
    }

    fn _connect(server_uri: &str) -> std::io::Result<TcpStream> {
        Ok(std::net::TcpStream::connect(server_uri)?)
    }

    fn check_socket(socket: &mut TcpStream, buf: &mut [u8], offset: usize) -> (usize, Option<messages::DataMessage>) {
        match socket.read(&mut buf[offset..]) {
            
            Ok(bytes) if bytes > 0 => {
                if let Ok((size, proto)) = get_proto_message(&buf[offset..bytes]) {
                    match proto.message_type() {
                        All::Message => {
                            (size, None)
                        },
                        _ => (size, None)
                    }
                }
                else {
                    (offset, None)
                }
            },
            Ok(bytes) => (bytes, None),        
            Err(_) => (0, None)
        }
    }

    pub fn publish(&self, subject: &str, data: &[u8]) {
        self.publish_channel.send(messages::Message::Publish(messages::PublishMessage::new(subject,data) )).expect("Failed to send msg");
    }

    pub fn subscribe(&self, subject: &str) -> Receiver<messages::DataMessage> {
        let (sender, receiver) = crossbeam::channel::bounded(1);
        self.publish_channel.send(messages::Message::Subscribe(messages::SubscribeMessage::new(subject, sender))).expect("failed to publish");
        receiver
    }
}