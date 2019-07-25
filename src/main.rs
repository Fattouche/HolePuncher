use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, Result, Socket, SocketEvent};
use log::info;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::SocketAddr;
use std::str;
use std::{thread, time};
mod connect;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 && args.len() != 4 {
        info!("Incorrect arguments ex: cargo run <listen_addr> <client_private_addr> <client_public_addr> <filename(sender only)>");
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

//Send the given file to the reciever
fn send_file(filename: &String, sender: Sender<Packet>, addr: SocketAddr) -> Result<()> {
    let packet = Packet::reliable_sequenced(addr, filename.clone().into_bytes(), None);
    sender.send(packet).unwrap();
    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);
    for data in f.lines() {
        info!("Sending data: {:?} to {:?}", data, addr);
        let data = data.expect("Unable to read data");
        let packet = Packet::reliable_sequenced(addr, data.into_bytes(), None);
        sender.send(packet).unwrap();
    }
    Ok(())
}

//After receiving a UDP packet from the sender, listen for a new connection.
fn recieve_file(reciever: Receiver<SocketEvent>, addr: SocketAddr) -> Result<()> {
    let result = reciever.recv_timeout(time::Duration::from_millis(10000));
    let mut location = String::from("uninitialized");
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
            info!("Failed to get filename from sender, quitting: {:?}", err);
            return Err(err).unwrap();
        }
    }
    let mut writer = File::open(location).unwrap();
    loop {
        let result = reciever.recv();
        match result {
            Ok(socket_event) => match socket_event {
                SocketEvent::Packet(packet) => {
                    let received_data: &[u8] = packet.payload();
                    writer.write_all(received_data)?;
                }
                SocketEvent::Timeout(timeout_event) => {
                    break;
                }
                _ => (),
            },
            Err(e) => {
                info!("Something went wrong when receiving, error: {:?}", e);
                return Err(e).unwrap();
            }
        }
    }
    Ok(())
}
