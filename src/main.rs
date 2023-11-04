#[allow(clippy::all)]
mod packet_streaming {
    tonic::include_proto!("packet_streaming");
}
mod args;
mod logger;
mod packet;
mod sniffer;

use packet_streaming::packet_streaming_client::PacketStreamingClient;
use packet_streaming::{Packet, PacketData, PacketHeader};

use args::Args;
use async_stream::stream;
use clap::Parser;
use colored::Colorize;
use futures_util::{pin_mut, StreamExt};
use packet::PacketInfo;
use sniffer::Sniffer;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut core = Sniffer::new(&args).unwrap();
    let mut client = match args.url {
        Some(ref url) => Some(
            PacketStreamingClient::connect(url.to_owned())
                .await
                .unwrap(),
        ),
        None => None,
    };

    logger::info("Sniffer started");

    let packet_stream = stream! {
        while let Ok(packet) = core.capture.next_packet() {
            match PacketInfo::from(&packet) {
                Some(info) => {
                    match core.blacklist {
                        Some(ref blacklist) => {
                            if blacklist.from.contains(&info.src_mac) {
                                logger::debug(format!("IGNORED: {}", info));
                                continue;
                            }
                            if blacklist.into.contains(&info.dst_mac) {
                                logger::debug(format!("IGNORED: {}", info));
                                continue;
                            }
                            if let Some(ip) = info.src_ip {
                                if blacklist.from.contains(&ip.to_string()) {
                                    logger::debug(format!("IGNORED: {}", info));
                                    continue;
                                }
                                if let Some(port) = info.src_port {
                                    if blacklist.from.contains(&format!("{}:{}", ip, port)) {
                                        logger::debug(format!("IGNORED: {}", info));
                                        continue;
                                    }
                                }
                            }
                            if let Some(ip) = info.dst_ip {
                                if blacklist.into.contains(&ip.to_string()) {
                                    logger::debug(format!("IGNORED: {}", info));
                                    continue;
                                }
                                if let Some(port) = info.dst_port {
                                    if blacklist.into.contains(&format!("{}:{}", ip, port)) {
                                        logger::debug(format!("IGNORED: {}", info));
                                        continue;
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                    logger::debug(format!("{}", info))
                }
                None => logger::debug(format!("[{}] Could not parse the packet", "???".red())),
            }

            if core.savefile.is_some() {
                core.savefile.as_mut().unwrap().write(&packet);
            }

            yield Packet {
                header: Some(PacketHeader {
                    ts_sec: packet.header.ts.tv_sec as u32,
                    ts_usec: packet.header.ts.tv_usec as u32,
                    caplen: packet.header.caplen,
                    len: packet.header.len,
                }),
                data: Some(PacketData {
                    data: packet.data.to_vec(),
                }),
            };
        }
    };

    if let Some(ref mut cli) = client {
        cli.run(packet_stream).await.unwrap();
    } else {
        pin_mut!(packet_stream);
        while (packet_stream.next().await).is_some() {}
    }
}
