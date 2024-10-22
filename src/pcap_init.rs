use pcap::{Capture, Device, Linktype};

pub struct Cap<T: pcap::State> {
    pub cap: Capture<T>,
}

impl Cap<pcap::Active> {
    pub fn open(device: Option<Device>) -> Cap<pcap::Active> {
        let device = match device {
            Some(d) => d,
            None => Cap::<pcap::Active>::find_default_device(),
        };

        let cap = Capture::from_device(device)
            .unwrap()
            .open()  
            .unwrap();
        Cap { cap }
    }

    fn find_default_device() -> Device {
        let devices = pcap::Device::list().unwrap();
        let device = devices.first().unwrap();
        device.clone()
    }

    pub fn next_packet(&mut self) -> Option<pcap::Packet<'_>> {
        self.cap.next_packet().ok() 
    }
}

impl Cap<pcap::Offline> {
    pub fn open_pcap(path: &str) -> Cap<pcap::Offline> {
        let cap = Capture::from_file(path)
            .unwrap();
        println!("{:?}",cap.get_datalink());
          
        Cap { cap }
    
    }
    pub fn next_packet(&mut self) -> Option<pcap::Packet<'_>> {
        self.cap.next_packet().ok() 
    }

    

    
}

