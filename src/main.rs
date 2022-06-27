mod parser;

use pcap::Device;
use clap::Parser;

fn main() {
    let args = parser::Args::parse();

    let dev = match args.device {
        Some(dev) => Device::from(dev.as_str()),
        None      => Device::lookup().expect("device lookup failed"),
    };

    println!("Using device \"{}\"", dev.name);

    let mut cap = pcap::Capture::from_device(dev)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    let mut savefile = match args.output {
        Some(file) => Some(cap.savefile(file).unwrap()),
        None       => None,
    };

    while let Ok(packet) = cap.next() {
        if args.verbose {
            println!("received packet! {:?}", packet);
        }
        if savefile.is_some() {
            savefile.as_mut().unwrap().write(&packet);
        }
    }
}
