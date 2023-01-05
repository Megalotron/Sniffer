#[allow(clippy::all)]
mod packet_streaming {
    tonic::include_proto!("packet_streaming");
}
mod args;
mod logger;
mod packet;
mod protocol;
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

    logger::get().info("Sniffer started");

    let stream = stream! {
        while let Ok(packet) = core.capture.next_packet() {
            match PacketInfo::from(&packet) {
                Some(info) => logger::get().debug(format!("{}", info)),
                None => logger::get().debug(format!("[{}] Could not parse the packet", "???".red())),
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
        cli.run(stream).await.unwrap();
    } else {
        pin_mut!(stream);
        while (stream.next().await).is_some() {}
    }
}
