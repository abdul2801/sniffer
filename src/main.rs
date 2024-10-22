use etherparse::{Ethernet2Header, SlicedPacket};
use packets::parse;

pub mod pcap_init;
mod packets;

fn main() {
    // let mutz  cap = pcap_init::Cap::<pcap::Active>::open(None);
    // loop {
    //     match cap.next_packet() {
    //         Some(packet) => println!("{:?}", packet),
    //         None => {
    //             println!("No packet received or an error occurred.");
    //             break; // You may want to handle this differently
    //         }
    //     }
    // }

    let mut cap: pcap_init::Cap<pcap::Offline> =
        pcap_init::Cap::<pcap::Offline>::open_pcap("ipv6-neig.pcap");
    
    loop {
        match cap.next_packet() {
            Some(packet) => {
                // get the payload of the ethernet frame
                let sliced = SlicedPacket::from_ethernet(packet.data).unwrap();
                // get ether type
                
                parse(sliced.link.unwrap());
            }

            None => {
                break;
            }
        }
    }
}
