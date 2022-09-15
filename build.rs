use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    tonic_build::compile_protos("api/packet_streaming.proto")?;
    Ok(())
}
