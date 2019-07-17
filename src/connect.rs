use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};
use std::net::SocketAddr;
use std::sync::mpsc;
use std::{thread, time};

pub fn connect(
    packet_sender: &Sender<Packet>,
    event_receiver: &Receiver<SocketEvent>,
    private_ip: String,
    public_ip: String,
) -> SocketAddr {
    let public_addr: SocketAddr = public_ip.parse().expect("Unable to parse socket address");
    let private_addr: SocketAddr = private_ip.parse().expect("Unable to parse socket address");
    if !hole_punch(public_addr, packet_sender, event_receiver) {
        return private_addr;
    }
    public_addr
}

fn hole_punch(
    addr: SocketAddr,
    packet_sender: &Sender<Packet>,
    event_receiver: &Receiver<SocketEvent>,
) -> bool {
    let (channel_sender, channel_reciever) = mpsc::channel();
    //one thread to recieve packets, main thread to send
    let local_event_reciever = event_receiver.clone();
    let thread = thread::spawn(move || {
        let result = local_event_reciever.recv_timeout(time::Duration::from_millis(5000));
        if result.is_err() {
            channel_sender.send(0);
        }
        match result {
            Ok(socket_event) => {
                match socket_event {
                    SocketEvent::Packet(packet) => {
                        let endpoint: SocketAddr = packet.addr();
                        let received_data: &[u8] = packet.payload();
                        if endpoint == addr {
                            //We actually got the udp packet from the other side
                            channel_sender.send(1);
                        }
                    }
                    SocketEvent::Connect(connect_event) => { /* a client connected */ }
                    SocketEvent::Timeout(timeout_event) => { /* a client timed out */ }
                }
            }
            Err(e) => {
                println!("Something went wrong when receiving, error: {:?}", e);
            }
        }
    });

    //Since this waits for the reciever to timeout, we don't need to join the threads
    loop {
        let data = vec![1];
        let unreliable = Packet::unreliable(addr, data);
        let millis = time::Duration::from_millis(500);
        thread::sleep(millis);
        packet_sender.send(unreliable).unwrap();
        let result = channel_reciever.try_recv();
        match result {
            Ok(code) => {
                if code == 0 {
                    return false;
                } else {
                    return true;
                }
            }
            Err(e) => println!("error reading from channel: {:?}", e),
        }
    }
}
