use std::net::TcpStream;
use std::io::Write;

use piraffe_common::make_connect;
use flatbuffers::*;


pub struct PiraffeClient {
    server_uri: String,
    client_name: String,
    handle: Option<std::thread::JoinHandle<()>>
}

impl PiraffeClient {
    pub fn new(server_uri: String, client_name: String) -> Self {
        PiraffeClient {
            server_uri,
            client_name,
            handle: None
        }
    }

    pub fn connect(&mut self)  {

        let server_uri = self.server_uri.to_string();
        self.handle = Some(std::thread::spawn({
            

            move || {
                let mut tcp_stream_res=  PiraffeClient::_connect(&server_uri);
                while tcp_stream_res.is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    tcp_stream_res = PiraffeClient::_connect(&server_uri);
                }

                let mut tcp_stream = tcp_stream_res.unwrap();

                loop {

                };
            }

        }));

        // match std::net::TcpStream::connect(&self.server_uri) {
        //     Ok(mut stream) => { 
        //         let mut builder = FlatBufferBuilder::new();
        //         let mut bytes: Vec<u8> = Vec::new();
        //         make_connect(&mut builder, &mut bytes, &self.client_name);
        //         stream.write(&bytes).expect("failed to write");


        //         self.stream = Some(stream);
                
        //         Ok(())
        //     },
        //     Err(_) => Err(())
        // }
    }

    fn _connect(server_uri: &str) -> std::io::Result<TcpStream> {
        Ok(std::net::TcpStream::connect(server_uri)?)
    }
}