mod packet_stream {
    tonic::include_proto!("packet_stream");
}
use packet_stream::packet_server::{Packet, PacketServer};
use packet_stream::*;

use futures_util::StreamExt;
use std::error::Error;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub struct PacketService;

#[tonic::async_trait]
impl Packet for PacketService {
    async fn send_packet(
        &self,
        request: Request<Streaming<SendPacketRequest>>,
    ) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();

        let cap = pcap::Capture::dead(pcap::Linktype::ETHERNET).unwrap();
        let mut savefile = cap.savefile("server.pcap").unwrap();
        while let Some(packet) = stream.next().await {
            let packet = packet.unwrap();
            let ts = packet.ts.unwrap();

            savefile.write(&pcap::Packet::new(
                &pcap::PacketHeader {
                    ts: libc::timeval {
                        tv_sec: ts.seconds,
                        tv_usec: ts.nanos,
                    },
                    caplen: packet.caplen as u32,
                    len: packet.len as u32,
                },
                &packet.data,
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
        .add_service(PacketServer::new(packet_service))
        .serve(addr)
        .await?;

    Ok(())
}
