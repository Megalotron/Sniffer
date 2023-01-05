use crate::protocol::Protocol;
use colored::Colorize;
use std::net::IpAddr;

pub struct PacketInfo {
    pub protocol: Protocol,
    pub src_mac: String,
    pub dst_mac: String,
    pub src_ip: Option<IpAddr>,
    pub dst_ip: Option<IpAddr>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub len: u32,
}

impl PacketInfo {
    pub fn from(packet: &[u8]) -> Option<Self> {
        let dst_mac = format!(
            "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
            packet[0], packet[1], packet[2], packet[3], packet[4], packet[5],
        );

        let src_mac = format!(
            "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
            packet[6], packet[7], packet[8], packet[9], packet[10], packet[11],
        );

        let packet_type = (packet[12], packet[13]);

        match packet_type {
            // IPV4
            (0x08, 0x00) => {
                let protocol = Protocol::from(packet[23])?;

                Some(Self {
                    protocol,
                    src_mac,
                    dst_mac,
                    src_ip: Some([packet[26], packet[27], packet[28], packet[29]].into()),
                    dst_ip: Some([packet[30], packet[31], packet[32], packet[33]].into()),
                    src_port: u16::from_str_radix(&format!("{:x}{:x}", packet[34], packet[35]), 16)
                        .ok(),
                    dst_port: u16::from_str_radix(&format!("{:x}{:x}", packet[36], packet[37]), 16)
                        .ok(),
                    len: packet.len() as u32,
                })
            }
            // IPV6
            (0x86, 0xdd) => {
                let protocol = Protocol::from(packet[20])?;

                Some(Self {
                    protocol,
                    src_mac,
                    dst_mac,
                    src_ip: Some(
                        [
                            packet[22], packet[23], packet[24], packet[25], packet[26], packet[27],
                            packet[28], packet[29], packet[30], packet[31], packet[32], packet[33],
                            packet[34], packet[35], packet[36], packet[37],
                        ]
                        .into(),
                    ),
                    dst_ip: Some(
                        [
                            packet[38], packet[39], packet[40], packet[41], packet[42], packet[43],
                            packet[44], packet[45], packet[46], packet[47], packet[48], packet[49],
                            packet[50], packet[51], packet[52], packet[53],
                        ]
                        .into(),
                    ),
                    src_port: u16::from_str_radix(&format!("{:x}{:x}", packet[54], packet[55]), 16)
                        .ok(),
                    dst_port: u16::from_str_radix(&format!("{:x}{:x}", packet[56], packet[57]), 16)
                        .ok(),
                    len: packet.len() as u32,
                })
            }
            // ETH
            (0x08, 0x06) => Some(Self {
                protocol: Protocol::Arp,
                src_mac,
                dst_mac,
                src_ip: None,
                dst_ip: None,
                src_port: None,
                dst_port: None,
                len: packet.len() as u32,
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for PacketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}{} -> {}{} ({} bytes) ",
            self.protocol,
            if let Some(src_ip) = self.src_ip {
                src_ip.to_string()
            } else {
                self.src_mac.to_string()
            },
            if let Some(src_port) = self.src_port {
                format!("{}{}", ":".red(), src_port)
            } else {
                "".to_string()
            },
            if let Some(dst_ip) = self.dst_ip {
                dst_ip.to_string()
            } else {
                self.dst_mac.to_string()
            },
            if let Some(dst_port) = self.dst_port {
                format!("{}{}", ":".red(), dst_port)
            } else {
                "".to_string()
            },
            self.len,
        )
    }
}
