mod schema_generated;

use tokio::net::UdpSocket;
use std::io;
use std::net::SocketAddr;
use flatbuffers;
use flatbuffers::FlatBufferBuilder;
use schema_generated::messages::{
    Message, MessageArgs,
    finish_message_buffer, get_root_as_message,
    letter, letterArgs, *
};

fn main() {
    println!("Hello, world!");

    let cons :Vec<SocketAddr> = Vec::new();

    echo_server();
}

#[tokio::main]
async fn echo_server() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1436];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        //println!("{:?} bytes received from {:?}", len, addr);

        // let len = sock.send_to(&buf[..len], addr).await?;
        // println!("{:?} bytes sent", len);
    }
}

// pub struct Message {
//     pub secret: [u8; 10],
//     pub time: u8,
//     pub message: [u8; 1436]
// }