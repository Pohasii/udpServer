use crate::client;
use crate::general_types::{ConnectionType};
use std::net::UdpSocket;
use std::sync::mpsc;
use std::time::SystemTime;

fn listener(socket: &UdpSocket, sender: mpsc::Sender<Sms>, conns: ConnectionType) {
    let mut buffer: [u8; 1400] = [0; 1400];
    loop {
        let (len, addr) = socket.recv_from(&mut buffer).expect("Didn't receive data");

        {
            let mut is_add = true;

            let mut connections = conns.lock().unwrap();
            for cl in connections.iter_mut() {
                if cl.addr.eq(&addr) {
                    is_add = false;
                    cl.date_of_last_message = SystemTime::now();
                }
            }

            if is_add {
                let adr = addr.clone();

                let mut cl = client::new(adr, 0);
                println!("add new client with addr: {:#?}", addr.clone());
                connections.push(cl);
            }
        }

        let message_to_channel = Sms {
            addr,
            data: buffer[..len].to_vec(),
        };

        sender
            .send(message_to_channel)
            .expect("couldn't send to channel");
    }
}
