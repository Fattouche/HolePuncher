use laminar::{Socket, Packet, SocketEvent, Result};
use crossbeam_channel::{unbounded, Receiver, SendError, Sender, TryRecvError};
use std::thread;
mod connect;


fn main() -> Result<()> {
    let mut socket = Socket::bind("127.0.0.1:8000")?;
    let private_ip = String::from("127.0.0.1:9000");
    let public_ip = String::from("127.0.0.1:9001");
    let is_sender = true;
    if is_sender{
        let (sender,addr) = connect::connect_as_sender(&socket, private_ip, public_ip);
        // send_file(sender, addr);
    }else{
        let (reciever,addr) = connect::connect_as_reciever(&socket, private_ip, public_ip);
        // recieve_file(reciever,addr);
    }
    
    
    Ok(())
}

fn transfer_file(sender: Sender<Packet>, reciever:Receiver<SocketEvent>, is_sender:bool){
    if is_sender{
        let file_name = String::from("test.txt");
        // send_file(filename, sender);
    }else{
        recieve_file();
    }
}

fn send_file(filename: String, ){

}

fn recieve_file(){

}