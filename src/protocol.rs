use colored::Colorize;

pub enum Protocol {
    Icmp,
    Igmp,
    Tcp,
    Arp,
    Udp,
}

impl Protocol {
    pub fn from(value: u8) -> Option<Protocol> {
        match value {
            1 | 58 => Some(Protocol::Icmp),
            2 => Some(Protocol::Igmp),
            6 => Some(Protocol::Tcp),
            17 => Some(Protocol::Udp),
            _ => None,
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Protocol::Igmp => "IGMP".cyan(),
            Protocol::Icmp => "ICMP".magenta(),
            Protocol::Tcp => "TCP".blue(),
            Protocol::Arp => "ARP".yellow(),
            Protocol::Udp => "UDP".green(),
        };

        write!(f, "{}", text)
    }
}
