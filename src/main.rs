use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Result, Socket, SocketEvent};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::SocketAddr;
use std::str;
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
        send_file(&filename, packet_sender, addr)?;
    } else {
        recieve_file(event_reciever, addr)?;
    }
    Ok(())
}

fn send_file(filename: &String, sender: Sender<Packet>, addr: SocketAddr) -> Result<()> {
    let packet = Packet::reliable_sequenced(addr, filename.clone().into_bytes(), None);
    sender.send(packet).unwrap();
    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);
    for data in f.lines() {
        println!("Sending data: {:?} to {:?}", data, addr);
        let data = data.expect("Unable to read data");
        let packet = Packet::reliable_sequenced(addr, data.into_bytes(), None);
        sender.send(packet).unwrap();
    }
    Ok(())
}

fn recieve_file(reciever: Receiver<SocketEvent>, addr: SocketAddr) -> Result<()> {
    let result = reciever.recv();
    let mut location = String::from("uninitialized");
    println!("RESULT: {:?}", result);
    match result {
        Ok(socket_event) => match socket_event {
            SocketEvent::Packet(packet) => {
                let filename: &[u8] = packet.payload();
                location = format!("{}-new", str::from_utf8(&filename).unwrap());
                let file = File::create(&location).unwrap();
            }
            _ => return Err("Error getting filename from sender").unwrap(),
        },
        Err(err) => {
            println!("Failed to get filename from sender, quitting: {:?}", err);
            return Err(err).unwrap();
        }
    }
    let mut writer = File::open(location).unwrap();
    loop {
        println!("Waiting ");
        let result = reciever.recv();
        match result {
            Ok(socket_event) => match socket_event {
                SocketEvent::Packet(packet) => {
                    let received_data: &[u8] = packet.payload();
                    println!("RECIEVED SOME SHIT:{:?}", received_data);
                    writer.write_all(received_data)?;
                }
                SocketEvent::Timeout(timeout_event) => {
                    break;
                }
                _ => (),
            },
            Err(e) => {
                println!("Something went wrong when receiving, error: {:?}", e);
                return Err(e).unwrap();
            }
        }
    }
    Ok(())
}
