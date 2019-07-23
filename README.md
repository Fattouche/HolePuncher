# Holepuncher

A simple module that lets you punch a hole through a users nat given that you know their public and private IP. 

Mostly just trying to learn Rust.

Example(using main):

1. If sender, create a file in the root of the project.
2. As sender run command: `cargo run <listen_addr> <client_private_addr> <client_public_addr> <filename>`
3. As reciever run command: `cargo run <listen_addr> <client_private_addr> <client_public_addr> <filename>`

In order to obtain the public IP, a central rendezvous server must be connected to by both clients. This is not included within this module.

`cargo run 127.0.0.1:8000 127.0.0.1:9000 127.0.0.1:9001 test.txt`