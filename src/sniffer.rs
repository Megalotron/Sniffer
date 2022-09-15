use crate::args::Args;
use crate::logger;
use std::error::Error;

pub struct Sniffer {
    pub capture: pcap::Capture<dyn pcap::Activated>,
    pub savefile: Option<pcap::Savefile>,
}

impl Sniffer {
    pub fn new(args: &Args) -> Result<Self, Box<dyn Error>> {
        let verbosity = match args.verbosity.as_str() {
            "debug" => logger::LogLevel::Debug,
            "info" => logger::LogLevel::Info,
            "warn" => logger::LogLevel::Warn,
            "error" => logger::LogLevel::Error,
            _ => panic!("Invalid verbosity level, choose beetwen [debug, info, warn, error]"),
        };

        logger::init()
            .stack("sniffer")
            .verbosity(verbosity)
            .logfile(&args.logfile)?
            .run();

        std::panic::set_hook(Box::new(|err| {
            if let Some(msg) = err.payload().downcast_ref::<&str>() {
                logger::get().error(msg);
            } else if let Some(msg) = err.payload().downcast_ref::<String>() {
                logger::get().error(msg);
            } else {
                logger::get().error(err);
            }
            std::process::exit(84);
        }));

        ctrlc::set_handler(|| {
            logger::get().warn("Sniffer killed by Ctrl c");
            std::process::exit(84);
        })
        .ok();

        if args.input.is_some() && args.device.is_some() {
            panic!("You can't use both input file and device input at the same time");
        }

        let dev = match args.device {
            Some(ref dev) => pcap::Device::from(dev.as_str()),
            None => pcap::Device::lookup()?.unwrap(),
        };

        let capture: pcap::Capture<dyn pcap::Activated> = match args.input {
            Some(ref input) => pcap::Capture::from(pcap::Capture::from_file(input)?),
            None => pcap::Capture::from(
                pcap::Capture::from_device(dev)?
                    .immediate_mode(true)
                    .open()?,
            ),
        };

        let savefile = match args.output {
            Some(ref file) => Some(capture.savefile(file)?),
            None => None,
        };

        Ok(Self { capture, savefile })
    }
}
