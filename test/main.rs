mod packet_streaming {
    tonic::include_proto!("packet_streaming");
}
use packet_streaming::packet_streaming_server::{PacketStreaming, PacketStreamingServer};
use packet_streaming::*;

use futures_util::StreamExt;
use std::error::Error;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub struct PacketService;

#[tonic::async_trait]
impl PacketStreaming for PacketService {
    async fn run(&self, request: Request<Streaming<Packet>>) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();

        let cap = pcap::Capture::dead(pcap::Linktype::ETHERNET).unwrap();
        let mut savefile = cap.savefile("server.pcap").unwrap();
        while let Some(packet) = stream.next().await {
            let packet = packet.unwrap();
            let packet_header = packet.header.unwrap();
            let packet_data = packet.data.unwrap();

            savefile.write(&pcap::Packet::new(
                &pcap::PacketHeader {
                    ts: libc::timeval {
                        tv_sec: packet_header.ts_sec as libc::time_t,
                        tv_usec: packet_header.ts_usec as libc::suseconds_t,
                    },
                    caplen: packet_header.caplen as u32,
                    len: packet_header.len as u32,
                },
                &packet_data.data,
            ));
        }
        savefile.flush().unwrap();
        Ok(tonic::Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let packet_service = PacketService {};

    Server::builder()
        .add_service(PacketStreamingServer::new(packet_service))
        .serve(addr)
        .await?;

    Ok(())
}
