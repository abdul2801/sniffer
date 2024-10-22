use serde::Serialize;



#[derive(Debug, PartialEq, Serialize)]
pub enum ARP {
    Request(ArpPacket),
    Reply(ArpPacket),
    Unknown
}

#[derive(PartialEq , Serialize , Debug)]
pub struct ArpPacket {
    pub source_mac : [u8; 6],
    pub source_ip : [u8; 4],
    pub dest_mac : [u8; 6],
    pub dest_ip : [u8; 4],
    pub op_code : u16
}
pub fn extract(payload: &[u8]) -> ARP {
    let arp_result = arp_parse::parse(payload);

    match arp_result {
        Ok(arp) => {
            let source_mac = match arp.sender_hardware_addr().try_into() {
                Ok(mac) => mac,
                Err(_) => return ARP::Unknown,
            };

            let source_ip = match arp.sender_protocol_addr().try_into() {
                Ok(ip) => ip,
                Err(_) => return ARP::Unknown,
            };

            let dest_mac = match arp.target_hardware_addr().try_into() {
                Ok(mac) => mac,
                Err(_) => return ARP::Unknown,
            };

            let dest_ip = match arp.target_protocol_addr().try_into() {
                Ok(ip) => ip,
                Err(_) => return ARP::Unknown,
            };

            let arp_packet = ArpPacket {
                source_mac,
                source_ip,
                dest_mac,
                dest_ip,
                op_code: arp.op_code(),
            };

            match arp.op_code() {
                arp_parse::OPCODE_REQUEST => ARP::Request(arp_packet),
                arp_parse::OPCODE_REPLY => ARP::Reply(arp_packet),
                _ => ARP::Unknown,
            }
        }
        Err(_) => {
            // error parsing ARP packet
            ARP::Unknown
        }
    }
}