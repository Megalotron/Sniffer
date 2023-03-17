mod args;

use args::Args;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dev = match args.device {
        Some(dev) => pcap::Device::from(dev.as_str()),
        None => pcap::Device::lookup()?.unwrap(),
    };

    println!("Using device \"{}\"", dev.name);

    let mut cap = pcap::Capture::from_device(dev)?
        .immediate_mode(true)
        .open()?;

    let mut savefile = match args.output {
        Some(file) => Some(cap.savefile(file)?),
        None => None,
    };

    while let Ok(packet) = cap.next_packet() {
        if args.verbose {
            let msg = format!("{packet:?}");
            println!("{msg}");
        }
        if savefile.is_some() {
            savefile.as_mut().unwrap().write(&packet);
        }
    }
    Ok(())
}
