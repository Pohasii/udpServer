
extern crate flatbuffers;

mod scheme_generated;

use tokio::net::UdpSocket;
use std::io;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::sync::mpsc;
use std::thread;
// use serde::{Serialize, Deserialize};
// use std::time::{Duration, SystemTime};
use flatbuffers::FlatBufferBuilder;
use crate::scheme_generated::messages::{MessArgs, finish_mess_buffer, get_root_as_mess};
use crate::scheme_generated::messages::Mess;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let sock:UdpSocket = UdpSocket::bind("127.0.0.1:8080").await?;

    //let mut cons:Clients; //Vec<SocketAddr> = Vec::new();

    //let (sender, receiver) = mpsc::channel();

    let mut builder = FlatBufferBuilder::new();

    let mut users: Vec<SocketAddr> = Vec::new();

    //let mut bytes: Vec<u8> = Vec::new();
    //let mut buf: Vec<u8> = Vec::new();
    let mut buf = [0; 1436];

    let mut message_count:i64 = 0;
    let mut message_count_sent:i64 = 0;

    loop {

        let (len, addr) = sock.recv_from(&mut buf).await?;

        let mut status = true;
        if users.len() < 1 {
            status = true
        } else {
            for user in users.iter() {
                if user.eq(&addr) {
                    status = false;
                }
            }
        }

        message_count += 1;
        if message_count % 10000 == 0 {
            println!("{}", message_count);
            println!("{}", message_count_sent);
        }
        //println!("{}, {}, {}", &buf.len(), &addr.to_string(), len);

        let mut data = read_mess(&buf[..len]);

        let mut current_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
        if status {
            current_addr = addr.clone();
            users.push(addr);
            println!("{:#?}", &users)
        }

        let mut ready_message:Vec<u8> = Vec::new();
        // make message
        make_mess(&mut builder, &mut ready_message, data.0, data.1, data.2);

        for user_addr in users.iter() {
            let adr = if status {&current_addr} else {&addr};
            if user_addr.ne(adr) {
                message_count_sent += 1;
                let len = sock.send_to(&ready_message, user_addr).await?;
            }
        }

        //let deserialized: Message = serde_json::from_slice(&buf).unwrap();

        //sender.send(deserialized.message);

        //let now = SystemTime::now();
        // buf.clear();
    }
    //echo_server();
}

// fn listener (sock:UdpSocket, sender:mpsc::Sender<T>) {
//     let mut buf = [0; 1436];
//     loop {
//         let (len, addr) = sock.recv_from(&mut buf).await?;
//
//         let deserialized: Message = serde_json::from_slice(&buf).unwrap();
//
//         sender.send(deserialized.message);
//
//         let now = SystemTime::now();
//     }
// }

// async fn echo_server() {
//
//     let mut buf = [0; 1436];
//     loop {
//         let (len, addr) = sock.recv_from(&mut buf).await?;
//         //println!("{:?} bytes received from {:?}", len, addr);
//
//         // let len = sock.send_to(&buf[..len], addr).await?;
//         // println!("{:?} bytes sent", len);
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Message {
//     pub secret: [u8; 10],
//     pub time: u8,
//     pub message: [u8; 1436]
// }
//
// pub struct Clients([Client; 50]);
//
// impl Clients {
//
//     fn search_client_by_addr (&self, addr: &SocketAddr) -> (bool, usize) {
//         if self.0.is_empty() {
//             return (false, 0)
//         } else {
//             for (index, client) in self.0.iter().enumerate() {
//                 let result = client.check_client_addr(addr);
//                 if result {
//                     return (result, index)
//                 }
//             }
//         }
//         (false, 0)
//     }
//
//     fn add_new_client_or_update (&mut self, addr: SocketAddr, is_exist: bool, index: usize) -> bool {
//         if is_exist {
//             self.0[index].addr = addr;
//             self.0[index].status = true;
//             self.0[index].date_of_last_message = SystemTime::now();
//
//             return true
//         }
//
//         for client in self.0.iter_mut() { // self.0.iter()
//             if client.status.ne(&true) && client.addr.to_string() == "0.0.0.0" {
//                 client.addr = addr;
//                 client.status = true;
//                 client.date_of_last_message = SystemTime::now();
//
//                 return true
//             }
//         }
//         false
//     }
//
//     fn process_client (&mut self, addr: SocketAddr) {
//
//         let (status, index) = self.search_client_by_addr(&addr);
//         if status {
//             self.add_new_client_or_update(addr,status,index);
//         } else {
//             self.add_new_client_or_update(addr,status,index);
//         }
//     }
// }
//
// pub struct Client {
//     pub addr: SocketAddr,
//     pub date_of_last_message: SystemTime,
//     pub status: bool,
// }
//
// impl Client {
//     fn check_client_addr (&self, addr:&SocketAddr) -> bool {
//         self.addr.eq(addr)
//     }
// }


// flatbuffers

pub fn make_mess (builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>, name: &str, id: u64, text: &str) {
    //reset the previously data
    dest.clear();

    builder.reset();

    let args = MessArgs {
        name: Some(builder.create_string(name)),
        id,
        text: Some(builder.create_string(text)),
    };

    let mess_offset = Mess::create(builder, &args);

    finish_mess_buffer(builder, mess_offset);

    let finished_data = builder.finished_data();

    dest.extend_from_slice(finished_data);
}

pub fn read_mess (buf: &[u8]) -> (&str, u64, &str) {

    //println!("{}", &buf.len());
    let message = get_root_as_mess(buf);

    let name = message.name().unwrap();

    let id = message.id();

    let text = message.text().unwrap();

    (name, id, text)
}