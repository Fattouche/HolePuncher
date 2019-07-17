use crossbeam_channel::{unbounded, Receiver, SendError, Sender, TryRecvError};
use laminar::{Packet, Result, Socket, SocketEvent};
use std::net::SocketAddr;
use std::thread;
mod connect;

fn main() -> Result<()> {
    let mut socket = Socket::bind("127.0.0.1:8000")?;
    let packet_sender = socket.get_packet_sender();
    let event_reciever = socket.get_event_receiver();
    let _thread = thread::spawn(move || socket.start_polling());
    let private_ip = String::from("127.0.0.1:9000");
    let public_ip = String::from("127.0.0.1:9001");
    let is_sender = true;
    let addr = connect::connect(&packet_sender, &event_reciever, private_ip, public_ip);
    let filename = String::from("test.txt");
    if is_sender {
        send_file(filename, packet_sender, addr);
    } else {
        recieve_file(filename, event_reciever, addr);
    }
    Ok(())
}

fn send_file(filename: String, sender: Sender<Packet>, addr: SocketAddr) {}

fn recieve_file(filename: String, reciever: Receiver<SocketEvent>, addr: SocketAddr) {}
