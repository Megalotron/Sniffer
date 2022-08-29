mod args;

use args::Args;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dev = match args.device {
        Some(dev) => pcap::Device::from(dev.as_str()),
        None => pcap::Device::lookup()?,
    };

    println!("Using device \"{}\"", dev.name);

    let mut cap = pcap::Capture::from_device(dev)?
        .immediate_mode(true)
        .open()?;

    let mut savefile = match args.output {
        Some(file) => Some(cap.savefile(file)?),
        None => None,
    };

    while let Ok(packet) = cap.next() {
        if args.verbose {
            println!("received packet! {:?}", packet);
        }
        if savefile.is_some() {
            savefile.as_mut().unwrap().write(&packet);
        }
    }
    Ok(())
}
