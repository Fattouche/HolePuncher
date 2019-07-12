use laminar::{Socket, Packet};
use crossbeam_channel::{unbounded, Receiver, SendError, Sender, TryRecvError};

pub fn connect(socket: &Socket, private_ip: String, public_ip: String) -> (Sender<Packet>, Reciever<SocketEvent>){
    let packet_sender = socket.get_packet_sender();
    let packet_reciever = socket.get_event_receiver();
    socket
}

pub fn hole_punch(){
    
}