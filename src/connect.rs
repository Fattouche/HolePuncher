use laminar::{Socket, Packet, SocketEvent};
use crossbeam_channel::{unbounded, Receiver, SendError, Sender, TryRecvError};
use std::net::SocketAddr;
use std::thread;

pub fn connect_as_sender(socket: &Socket, private_ip: String, public_ip: String) -> (Sender<Packet>, SocketAddr){
    let (packet_sender, event_reciever, addr) = connect(socket, private_ip, public_ip);
    (packet_sender, addr)
}

pub fn connect_as_reciever(socket: &Socket, private_ip: String, public_ip: String) -> (Receiver<SocketEvent>, SocketAddr){
    let (packet_sender, event_reciever, addr) = connect(socket, private_ip, public_ip);
    (event_reciever, addr)
}

fn connect(socket: &Socket, private_ip: String, public_ip: String) -> (Sender<Packet>, Receiver<SocketEvent>, SocketAddr){
    let packet_sender = socket.get_packet_sender();
    let event_reciever = socket.get_event_receiver();
    let public_addr: SocketAddr = public_ip.parse().expect("Unable to parse socket address");
    let private_addr: SocketAddr = private_ip.parse().expect("Unable to parse socket address");
    if !hole_punch(public_addr, packet_sender, event_reciever){
        return (packet_sender, event_reciever, private_addr);
    }
    (packet_sender, event_reciever,public_addr)
}

fn hole_punch(addr: SocketAddr, packet_sender: Sender<Packet>, packet_reciever: Receiver<SocketEvent>) -> bool{
    let data = vec![1];
    let unreliable = Packet::unreliable(addr, data);
    let mut children = vec![];
    //one thread to send packets, one to recieve
    // children.push(thread::spawn(move || {
    //     for
    // }));
    true
}