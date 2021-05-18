use std::net::{SocketAddr};
use std::net::{IpAddr, Ipv4Addr};

pub struct Setting {
    port: u16,
    ip: Ipv4Addr,
}

pub fn new (ip: Ipv4Addr, port: u16) -> Setting {
    Setting{
        port,
        ip,
    }
}

pub fn new_default () -> Setting {
    Setting{
        port: 0u16,
        ip: Ipv4Addr::new(127, 0, 0, 1), }
}

impl Setting {
    pub fn set_port(&mut self, port: u16) {
        self.port = port
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn set_ip(&mut self, ip:Ipv4Addr) {
        self.ip = ip
    }

    pub fn get_ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub fn get_address(&self) -> SocketAddr {
        // IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        SocketAddr::new(IpAddr::V4(self.get_ip()), self.get_port())
    }
}