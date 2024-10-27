use serde::Serialize;

use super::icmp::ICMP;

#[derive(Debug, PartialEq)]
pub enum ipv4 {
    icmp(ICMP),
    tcp,
    udp,
    Unknown(Vec<u8>),
}