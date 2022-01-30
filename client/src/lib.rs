use std::net::TcpStream;
use std::io::Write;

use messages::{Message, PublishMessage};
use piraffe_common::{make_connect, make_publish};
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
            tcp_tream.write(&vec).expect("failed to send message");
        },
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
                let mut builder=  FlatBufferBuilder::new();
                let mut tcp_stream_res=  PiraffeClient::_connect(&s_uri);
                while tcp_stream_res.is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    tcp_stream_res = PiraffeClient::_connect(&s_uri);
                }

                let mut tcp_stream = tcp_stream_res.unwrap();

                let mut bytes: Vec<u8> = Vec::new();
                make_connect(&mut builder, &mut bytes, &c_name);
                tcp_stream.write(&bytes).expect("failed to write");

                loop {
                    if let Ok(msg) = receiver.try_recv() {
                        send_encoded(&mut tcp_stream, &msg, &mut builder);
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

    pub fn publish(&self, subject: &str, data: &[u8]) {
        self.publish_channel.send(messages::Message::Publish(messages::PublishMessage::new(subject,data) )).expect("Failed to send msg");
    }
}