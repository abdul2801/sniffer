use super::{arp, icmp::ICMP, Ipv4::{self}};
use serde::Serialize;

#[derive(Debug, PartialEq)]
pub enum Ether {
    Arp(arp::ARP),
    IPv4(Ipv4::ipv4),
    IPv6(),
    Unknown(Vec<u8>),
}