use std::string::ParseError;

use etherparse::{Icmpv4Header, Icmpv4Type};
use pcap::Error;
use serde::Serialize;

use crate::error::{ErrorType, Errors};




pub fn extract(data : &[u8]) -> Result<ICMP, Errors> {
    if let Ok((icmp, rem)) = Icmpv4Header::from_slice(data) { 
        Ok(ICMP {
            hdr : icmp,
            data : rem.to_vec()
        })
    }
    else {
        Err(Errors::new(ErrorType::ParseError, "Failed to parse ICMP packet"))


    }
    
    
    
}

#[derive(Debug , PartialEq )]
pub struct ICMP {
    pub hdr : Icmpv4Header,
    pub data : Vec<u8>
}