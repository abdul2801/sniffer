mod arp;

use etherparse::{err::LenError, EtherType, Ethernet2Header, Ethernet2Slice, LinkSlice, SlicedPacket};
use pcap::Linktype;

pub fn parse(link : LinkSlice) {
    
    match link {
        LinkSlice::Ethernet2(eth) => match parse_ethernet(eth) {
            Ok(eth) => {
                // println!("{:?}", eth);
            }
            Err(e) => {
                println!("Error parsing ethernet: {:?}", e);
            }
            
        }
        _ => println!("Unknown link type"),
    }
}

fn parse_ethernet(eth: Ethernet2Slice) -> Result<Ethernet2Header , LenError> {

    let header = Ethernet2Header::from_slice(&eth.slice())?;

    
    
    match eth.ether_type() {
       EtherType::ARP => {
           println!("ARP packet");
            // parse ARP packet from ethparse from eth.payload() and print it
            let arp = arp::extract(eth.payload().payload);
            println!("{:?}", arp);


           Ok(header.0)
       }
       EtherType::IPV4 => {
            println!("IPv4 packet");
            Ok(header.0)
        }
        EtherType::IPV6 => {
            println!("IPv6 packet");
            Ok(header.0)
        }
        _ => {
            println!("Unknown packet type");
            Ok(header.0)
        }
    }


    
}

