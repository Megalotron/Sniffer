use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// URL of the grpc server to send the pcap data stream
    #[clap(short, long, value_parser)]
    pub url: Option<String>,

    /// Read packets from a pcap file instead of a network interface
    #[clap(short, long, value_parser)]
    pub read: Option<String>,

    /// Write captured packets on a pcap file
    #[clap(short, long, value_parser)]
    pub write: Option<String>,

    /// Set the verbosity level
    #[clap(short, long, value_parser, default_value = "info")]
    pub verbosity: String,

    /// If set, the logs will be save on the provided file
    #[clap(short, long, value_parser)]
    pub logfile: Option<String>,

    /// Use a specific network interface instead of the default one
    #[clap(short, long, value_parser)]
    pub interface: Option<String>,
}
