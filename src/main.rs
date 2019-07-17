use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Result, Socket, SocketEvent};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::net::SocketAddr;
use std::thread;
mod connect;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 && args.len() != 4 {
        println!("Incorrect arguments ex: cargo run <listen_addr> <client_private_addr> <client_public_addr> <filename(sender only)>");
        return Ok(());
    }
    let mut socket = Socket::bind(&args[1])?;
    let private_ip = String::from(&args[2]);
    let public_ip = String::from(&args[3]);
    let filename: String;
    if args.len() > 4 {
        filename = String::from(&args[4]);
    } else {
        filename = String::from("");
    }
    let packet_sender = socket.get_packet_sender();
    let event_reciever = socket.get_event_receiver();
    let _thread = thread::spawn(move || socket.start_polling());
    let addr = connect::connect(&packet_sender, &event_reciever, private_ip, public_ip);
    if filename != "" {
        send_file(filename, packet_sender, addr);
    } else {
        recieve_file(filename, event_reciever, addr);
    }
    Ok(())
}

fn send_file(filename: String, sender: Sender<Packet>, addr: SocketAddr) {
    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut buffer = [0; 1024];
    for data in f.lines() {
        let data = data.expect("Unable to read data");
        let packet = Packet::reliable_sequenced(addr, data.into_bytes(), None);
        sender.send(packet).unwrap();
    }
}

fn recieve_file(filename: String, reciever: Receiver<SocketEvent>, addr: SocketAddr) {}
