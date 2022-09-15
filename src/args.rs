use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// URL of the grpc server to send the pcap data stream
    #[clap(short, long, value_parser)]
    pub url: Option<String>,

    /// Path to a pcap input file where to load packets
    #[clap(short, long, value_parser)]
    pub input: Option<String>,

    /// Path to a pcap output file where to save captured packets
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// Set the verbosity level
    #[clap(short, long, value_parser, default_value = "info")]
    pub verbosity: String,

    /// If set, the logs will be save on the provided file
    #[clap(short, long, value_parser)]
    pub logfile: Option<String>,

    /// The provided network device will be used instead of the default one
    #[clap(short, long, value_parser)]
    pub device: Option<String>,
}
