use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// IP adress of the grpc server
    #[clap(short, long, value_parser)]
    pub ip: Option<String>,

    /// Port of the grpc server
    #[clap(short, long, value_parser, default_value_t = 50051)]
    pub port: u16,

    /// Path to a pcap output file where to save captured packets
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// If provided, the captured packets will be displayed in the shell
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,

    /// If provided, the sniffer will use the provided interface instead of the default one
    #[clap(short, long, value_parser)]
    pub device: Option<String>,
}
