use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// URL of the grpc server to send the pcap data stream
    #[clap(short, long, value_parser)]
    pub url: Option<String>,

    /// Path to a pcap output file where to save captured packets
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// The captured packets will be displayed in the shell
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,

    /// The provided network device will be used instead of the default one
    #[clap(short, long, value_parser)]
    pub device: Option<String>,
}
