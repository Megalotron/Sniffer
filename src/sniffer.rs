use crate::args::Args;
use crate::logger;
use std::error::Error;

#[derive(serde::Deserialize)]
pub struct Blacklist {
    pub from: Vec<String>,
    pub into: Vec<String>,
}

/// `Sniffer` is a struct that contains a `capture` field of type `pcap::Capture<dyn pcap::Activated>`
/// and a `savefile` field of type `Option<pcap::Savefile>`.
///
/// The `capture` field is a `pcap::Capture` object that is created by the
/// `pcap::Capture::from_device()` function.
///
/// Properties:
///
/// * `capture`: This is the pcap::Capture object that we'll use to capture packets.
/// * `savefile`: This is the file that the sniffer will save the packets to.
pub struct Sniffer {
    pub capture: pcap::Capture<dyn pcap::Activated>,
    pub savefile: Option<pcap::Savefile>,
    pub blacklist: Option<Blacklist>,
}

impl Sniffer {
    /// It initializes the logger, sets the panic hook, sets the SIGINT handler, checks if the user
    /// wants to read from a file or a network interface, and then creates a pcap capture object
    ///
    /// Arguments:
    ///
    /// * `args`: &Args
    ///
    /// Returns:
    ///
    /// A new instance of the Sniffer struct.
    pub fn new(args: &Args) -> Result<Self, Box<dyn Error>> {
        logger::set_stack("sniffer");
        logger::set_verbosity(args.verbosity);
        logger::set_logfile(&args.logfile)?;
        logger::init();

        std::panic::set_hook(Box::new(|err| {
            if let Some(msg) = err.payload().downcast_ref::<&str>() {
                logger::error(msg);
            } else if let Some(msg) = err.payload().downcast_ref::<String>() {
                logger::error(msg);
            } else {
                logger::error(err);
            }
            std::process::exit(84);
        }));

        ctrlc::set_handler(|| {
            print!("\r");
            logger::warn("Sniffer killed by ^C");
            std::process::exit(84);
        })
        .ok();

        if args.read.is_some() && args.interface.is_some() {
            panic!("You can't both read packets from an input file and a network interface");
        }

        let dev = match args.interface {
            Some(ref dev) => pcap::Device::from(dev.as_str()),
            None => pcap::Device::lookup()?.unwrap(),
        };

        let capture: pcap::Capture<dyn pcap::Activated> = match args.read {
            Some(ref input) => pcap::Capture::from(pcap::Capture::from_file(input)?),
            None => pcap::Capture::from(
                pcap::Capture::from_device(dev)?
                    .immediate_mode(true)
                    .open()?,
            ),
        };

        let savefile = match args.write {
            Some(ref file) => Some(capture.savefile(file)?),
            None => None,
        };

        let blacklist: Option<Blacklist> = match args.blacklist {
            Some(ref file) => {
                let data = std::fs::read_to_string(file)?;
                let conf: Blacklist = match toml::from_str(&data) {
                    Ok(blacklist) => blacklist,
                    Err(err) => {
                        logger::error(format!("Could not parse the blacklist file: {}", err));
                        std::process::exit(84);
                    }
                };

                Some(conf)
            }
            None => None,
        };

        Ok(Self { capture, savefile, blacklist })
    }
}
