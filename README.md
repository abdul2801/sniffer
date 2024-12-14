# Rust Packet Sniffer

## Overview
This is a Rust-based packet sniffer designed to capture and parse network packets in real time. It features a terminal-based user interface (TUI) built with `ratatui` (formerly `tui-rs`) for easy navigation and visualization of packet data.

## Features
- **Packet Capturing**: Uses `pcap` to capture packets from a specified network interface.
- **Packet Parsing**: Parses common protocols such as Ethernet, IPv4, IPv6, TCP, UDP, and more.
- **Real-Time Display**: Displays captured packets in a live updating TUI.
- **Filter Support**: Apply filters to capture specific types of packets (e.g., HTTP, DNS, etc.).
- **Statistics**: Displays statistics like packet count and protocol breakdown.

## Installation

### Prerequisites
2. **Libpcap**: Ensure `libpcap` is installed on your system.
   - For Linux: `sudo apt install libpcap-dev`
   - For macOS: `brew install libpcap`
   - For Windows: Use [Npcap](https://nmap.org/npcap/).

### Clone the Repository
```bash
git clone <repository-url>
cd rust-packet-sniffer
```

### Build the Project
```bash
cargo build --release
```

### Run the Application
```bash
sudo ./target/release/sniffer
```
(Note: Root privileges or administrator access may be required to access network interfaces.)


## Future Enhancements
- Add support for more protocols like ARP, ICMP, and HTTP.
- Save captured packets to a file (e.g., `.pcap` format).
- Add more advanced filtering options in the TUI.
- Enable remote monitoring by exporting data over a network.


