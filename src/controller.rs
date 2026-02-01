use std::net::UdpSocket;
use bx::network::address::Address;

use crate::package::ArtNetDmxPacket;

pub struct ArtNetController {
    socket: UdpSocket,
    node_host: Address,
    sequence: u8,
}

impl ArtNetController {
    /// default artnet port = 6454
    pub fn new(host: Address) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        Ok(ArtNetController {
            socket,
            node_host: host,
            sequence: 0,
        })
    }

    pub fn send_dmx(&mut self, mut packet: ArtNetDmxPacket) -> Result<(), Box<dyn std::error::Error>> {
        packet.set_sequence(self.sequence);
        self.sequence = self.sequence.wrapping_add(1);

        let bytes = packet.to_bytes();
        self.socket.send_to(&bytes, self.node_host.to_string())?;

        Ok(())
    }
}

