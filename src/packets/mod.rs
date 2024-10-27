mod arp;
mod icmp;
mod ether;
mod Ipv4;

use ether::Ether;
use etherparse::{err::{ipv4, LenError}, EtherType, Ethernet2Header, Ethernet2Slice, IpNumber, Ipv4Header, Ipv4HeaderSlice, LinkSlice, SlicedPacket};
use pcap::Linktype;

use crate::error::ErrorType;

pub fn parse(link : LinkSlice) {
    
    match link {
        LinkSlice::Ethernet2(eth) => match parse_ethernet(eth) {
            Ok(eth) => {
                println!("{:?}", eth);
            }
            Err(e) => {
                println!("Error parsing ethernet: {:?}", e);
            }
            
        }
        _ => println!("Unknown link type"),
    }
}

fn parse_ethernet(eth: Ethernet2Slice) -> Result<Ether , LenError> {

    let header = Ethernet2Header::from_slice(&eth.slice())?;

    
    
    match eth.ether_type() {
        
       EtherType::ARP => {
           println!("ARP packet");
            // parse ARP packet from ethparse from eth.payload() and print it
            let arp = arp::extract(eth.payload().payload);
            
            Ok(Ether::Arp(arp))

           
       }
       EtherType::IPV4 => {
            let pkt = parse_ipv4(eth.payload().payload);
            match pkt {
                Ok(pkt) => {
                    Ok(Ether::IPv4(pkt))
                }
                Err(e) => {
                    println!("Error parsing ipv4: {:?}", e);
                    Ok(Ether::Unknown(eth.payload().payload.to_vec()))
                }
            }
        }
        EtherType::IPV6 => {
            println!("IPv6 packet");
            Ok(Ether::IPv6())
        }
        _ => {
            println!("Unknown packet type");
            let payload = eth.payload().payload;
            // convert payload to hex string
            
            Ok(Ether::Unknown(payload.to_vec()))
    }
}


    
}

fn parse_ipv4(payload : &[u8])  -> Result<Ipv4::ipv4, ErrorType> {
    let ipv4 = Ipv4Header::from_slice(payload);
    
    

    match ipv4 {
        Ok((ipv4,rem)) =>  {
            let protocol = ipv4.protocol;
            
            let pkt = match protocol {
                IpNumber::ICMP => {
                    match icmp::extract(rem) {
                        Ok(icmp) => {
                            Ipv4::ipv4::icmp(icmp)
                            
                            
                        }
                        Err(e) => {
                            println!("Error parsing icmp: {:?}", e);
                            
                            Ipv4::ipv4::Unknown(rem.to_vec())
                        }
                    }
                }
                IpNumber::TCP => {
                    Ipv4::ipv4::tcp
                }
                IpNumber::UDP => {
                    Ipv4::ipv4::udp
                }
                _ => {
                    Ipv4::ipv4::Unknown(rem.to_vec())
                }
            };

            Ok(pkt)


            
        }
        Err(e) => {
            println!("Error parsing ipv4: {:?}", e);
            Err(ErrorType::ParseError)
        }
    }

}

