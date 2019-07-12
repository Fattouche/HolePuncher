use laminar::{Socket, Packet, SocketEvent, Result};
use crossbeam_channel::{unbounded, Receiver, SendError, Sender, TryRecvError};
mod connect;

fn main() -> Result<()> {
    let mut socket = Socket::bind("127.0.0.1:8000")?;
    let private_ip = String::from("127.0.0.1:9000");
    let public_ip = String::from("127.0.0.1:9001");
    let is_sender = true;
    let (sender,reciever) = connect::connect(&socket, private_ip, public_ip);
    transfer_file(sender, reciever, is_sender);
    Ok(())
}

fn transfer_file(sender: Sender<Packet>, reciever:Receiver<SocketEvent>, is_sender:bool){
    if is_sender{
        let file_name = String::from("test.txt");
        send_file(filename, sender);
    }else{
        recieve_file();
    }
}

fn send_file(filename: String, ){

}

fn recieve_file(){

}