mod args;
mod logger;
mod sniffer;
mod packet_stream {
    tonic::include_proto!("packet_stream");
}

use packet_stream::packet_client::PacketClient;
use packet_stream::SendPacketRequest;

use args::Args;
use async_stream::stream;
use clap::Parser;
use futures_util::{pin_mut, StreamExt};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut core = sniffer::Sniffer::new(&args).unwrap();
    let mut client = match args.url {
        Some(ref url) => Some(PacketClient::connect(url.to_owned()).await.unwrap()),
        None => None,
    };

    logger::get().info(format!("Sniffer started: {:?}", args));

    let stream = stream! {
        while let Ok(packet) = core.capture.next_packet() {
            logger::get().debug(format!("{:?}", packet));
            if core.savefile.is_some() {
                core.savefile.as_mut().unwrap().write(&packet);
            }
            yield SendPacketRequest {
                ts: Some(prost_types::Timestamp {
                    seconds: packet.header.ts.tv_sec,
                    nanos: packet.header.ts.tv_usec,
                }),
                caplen: packet.header.caplen,
                len: packet.header.len,
                data: packet.data.to_vec(),
            };
        }
    };

    if let Some(ref mut cli) = client {
        cli.send_packet(stream).await.unwrap();
    } else {
        pin_mut!(stream);
        while (stream.next().await).is_some() {}
    }
}
