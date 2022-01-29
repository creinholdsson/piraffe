use std::{net::{TcpListener, TcpStream}, thread, hash::Hasher};
use std::io::Read;

use crossbeam::channel::{Sender, Receiver, unbounded, bounded};

mod coordinator;
use coordinator::*;
mod clientlistener;
use clientlistener::*;
use piraffe_common::{MessageFromCoordinator, MessageToCoordinator};


fn main() {
    env_logger::init();
    log::debug!("Starting server");

    let address = "0.0.0.0:13377";
    let listener = TcpListener::bind(address).expect("Failed to bind socket");
    log::info!("Server is now listening on {}", address);


    let from_clients: (Sender<MessageToCoordinator>, Receiver<MessageToCoordinator>)  = unbounded();
    let to_clients: (Sender<MessageFromCoordinator>, Receiver<MessageFromCoordinator>) = unbounded();

    let mut coordinater = Coordinator::new(to_clients.0, from_clients.1);

    
    thread::spawn(move|| {
        log::debug!("Starting coordinator");
        coordinater.run();
    });

    for stream in listener.accept() {
        log::debug!("Client connected from {}", stream.1);
        let to_coordinator = from_clients.0.clone();
        let from_coordinator = to_clients.1.clone();
        let mut client_listener = ClientListener::new(from_coordinator, to_coordinator, stream.0);
        thread::spawn(move || {
            log::debug!("Spawning client listener");
            client_listener.run();          
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(2));
}
