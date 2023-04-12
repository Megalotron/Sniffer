use colored::Colorize;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::udp::UdpPacket;
use std::net::IpAddr;

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

/// The first field is a string that contains the name of the protocol. The next two fields are strings
/// that contain the source and destination MAC addresses. The next two fields are options of IP
/// addresses that contain the source and destination IP addresses. The next two fields are options of
/// numbers that contain the source and destination ports. The last field is a number that contains the
/// length of the packet.
///
/// Properties:
///
/// * `protocol`: The protocol of the packet.
/// * `src_mac`: The source MAC address
/// * `dst_mac`: The destination MAC address
/// * `src_ip`: The source IP address of the packet.
/// * `dst_ip`: The destination IP address
/// * `src_port`: The source port of the packet.
/// * `dst_port`: The destination port of the packet.
/// * `len`: The length of the packet in bytes.
pub struct PacketInfo {
    pub protocol: String,
    pub src_mac: String,
    pub dst_mac: String,
    pub src_ip: Option<IpAddr>,
    pub dst_ip: Option<IpAddr>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub len: u32,
}

impl PacketInfo {
    /// > If the packet is an IPv4 packet, parse it as an IPv4 packet, otherwise if it's an IPv6 packet,
    /// parse it as an IPv6 packet, otherwise if it's an ARP packet, parse it as an ARP packet,
    /// otherwise parse it as an Ethernet packet.
    ///
    /// Arguments:
    ///
    /// * `packet`: &[u8] - The packet to parse
    ///
    /// Returns:
    ///
    /// A `PacketInfo` struct containing all informations about the packet.
    pub fn from(packet: &[u8]) -> Option<Self> {
        let ethernet = EthernetPacket::new(packet)?;

        match ethernet.get_ethertype() {
            EtherTypes::Ipv4 => {
                let ipv4 = Ipv4Packet::new(ethernet.payload())?;

                let (src_port, dst_port) = match ipv4.get_next_level_protocol() {
                    IpNextHeaderProtocols::Udp => {
                        let udp = UdpPacket::new(ipv4.payload())?;

                        (Some(udp.get_source()), Some(udp.get_destination()))
                    }
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(ipv4.payload())?;

                        (Some(tcp.get_source()), Some(tcp.get_destination()))
                    }
                    _ => (None, None),
                };

                Some(PacketInfo {
                    protocol: ipv4.get_next_level_protocol().to_string(),
                    src_mac: ethernet.get_source().to_string(),
                    dst_mac: ethernet.get_destination().to_string(),
                    src_ip: Some(ipv4.get_source().into()),
                    dst_ip: Some(ipv4.get_destination().into()),
                    src_port,
                    dst_port,
                    len: ipv4.payload().len() as u32,
                })
            }
            EtherTypes::Ipv6 => {
                let ipv6 = Ipv6Packet::new(ethernet.payload())?;

                let (src_port, dst_port) = match ipv6.get_next_header() {
                    IpNextHeaderProtocols::Udp => {
                        let udp = UdpPacket::new(ipv6.payload())?;

                        (Some(udp.get_source()), Some(udp.get_destination()))
                    }
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(ipv6.payload())?;

                        (Some(tcp.get_source()), Some(tcp.get_destination()))
                    }
                    _ => (None, None),
                };

                Some(PacketInfo {
                    protocol: ipv6.get_next_header().to_string(),
                    src_mac: ethernet.get_source().to_string(),
                    dst_mac: ethernet.get_destination().to_string(),
                    src_ip: Some(ipv6.get_source().into()),
                    dst_ip: Some(ipv6.get_destination().into()),
                    src_port,
                    dst_port,
                    len: ipv6.payload().len() as u32,
                })
            }
            EtherTypes::Arp => {
                let arp = ArpPacket::new(ethernet.payload())?;
                Some(PacketInfo {
                    protocol: ethernet.get_ethertype().to_string(),
                    src_mac: ethernet.get_source().to_string(),
                    dst_mac: ethernet.get_destination().to_string(),
                    src_ip: Some(arp.get_sender_proto_addr().into()),
                    dst_ip: Some(arp.get_target_proto_addr().into()),
                    src_port: None,
                    dst_port: None,
                    len: arp.payload().len() as u32,
                })
            }
            _ => Some(PacketInfo {
                protocol: ethernet.get_ethertype().to_string(),
                src_mac: ethernet.get_source().to_string(),
                dst_mac: ethernet.get_destination().to_string(),
                src_ip: None,
                dst_ip: None,
                src_port: None,
                dst_port: None,
                len: ethernet.payload().len() as u32,
            }),
        }
    }
}

impl std::fmt::Display for PacketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}{} {} {}{} ({} bytes) ",
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
            "->".blue(),
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
