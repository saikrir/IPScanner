use std::{ ops::Range, sync::mpsc, thread, time::Duration};

use pinger::{ping, PingOptions, PingResult};

pub struct InputIp {
    ip_triplet: [u8;3],
    scan_range: Range<u8>
}

impl InputIp {
    pub fn parse(input_triplet: &str, start: u8, end:u8) -> Self {
        let vec: Vec<u8> = input_triplet.split(".").map(|s| s.parse::<u8>().unwrap()).collect();
        let range = start..end;
        Self {
            ip_triplet: [vec[0], vec[1], vec[2]],
            scan_range: range
        }
    }

    pub fn scan_range(&self) -> Range<u8> {
        self.scan_range.clone()
    }
}

impl ToString for InputIp {
    fn to_string(&self) -> String {
       format!("{}.{}.{}", self.ip_triplet[0], self.ip_triplet[1], self.ip_triplet[2])
    }
}

pub struct IpAddressRange{
    pub ip_addresses: Vec<String>,
}

impl IpAddressRange {
    pub fn new() -> Self {
        let v: Vec<String> = Vec::new();
        Self{
            ip_addresses: v
        }
    }

    pub fn generate(&mut self, ip_input: &InputIp) {
        self.ip_addresses.clear();
        let ip_prefix = ip_input.to_string();
        for i in ip_input.scan_range() {
            let ip_address = format!("{}.{}", ip_prefix, i);
            self.ip_addresses.push(ip_address);
        }
    }

    pub fn ip_count(&self) -> usize {
        self.ip_addresses.len()
    }

}


trait Pinger {
    fn ping(&self, ip_address: &str)-> Result<(), &str>;
}


fn pingOptions(ping_count: u8, interval:Duration, ip_address: &str) -> PingOptions {
    let ping_options = PingOptions::new(ip_address, interval, None);
    ping_options.with_raw_arguments(vec!["-c",ping_count.to_string().as_str()])
}

pub struct IpPinger {
    ip_addresses: Vec<String>,
    ping_count: u8,
    ping_interval: Duration, 
}

impl IpPinger {
    pub fn new(ip_addresses: Vec<String>, ping_count: u8, ping_interval: u64) ->Self {
        Self{
            ip_addresses,
            ping_count,
            ping_interval: Duration::from_secs(ping_interval)
        }
    }

    pub fn ping_in_range(&self) -> Vec<String>{
       
        let (tx, rx) = mpsc::channel::<String>();

        for ip_address in self.ip_addresses.clone() {
 
            let local_tx = tx.clone();
            
            thread::spawn(move || {
                let ping_opts = pingOptions(1, Duration::from_secs(1), &ip_address);

                let stream = ping(ping_opts).expect("Error pinging");
        
                let message =  stream.iter().next().unwrap();
        
                match message {
                    PingResult::Pong(_,_ ) => local_tx.send(ip_address.to_owned()).unwrap(),
                    _ => {}, 
                }
            });
        }
        drop(tx);
        
        rx.iter().collect()
    }

}

impl Pinger for IpPinger {
    fn ping(&self, ip_address: &str)-> Result<(), &str> {
        let ping_opts = pingOptions(self.ping_count, self.ping_interval, ip_address);

        let stream = ping(ping_opts).expect("Error pinging");

        let message =  stream.iter().next().unwrap();

        match message {
            PingResult::Pong(_,_ ) => Ok(()),
            _ => Err("Some Error occured") 
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_parses() {
        let result = InputIp::parse("192.168.86.127", 1, 255);
        assert_eq!([192,168,86], result.ip_triplet);
        println!("{}", result.to_string())
    }

    #[test]
    fn ip_count_is_correct() {
        let result = InputIp::parse("192.168.86.127", 1, 255);
        let mut ip_addr_range = IpAddressRange::new();
        ip_addr_range.generate(&result);
        assert_eq!(254, ip_addr_range.ip_count());
    }

    #[test]
    fn ip_ping_works() {
        let result = InputIp::parse("192.168.86.127", 1, 50);
        let mut ip_addr_range = IpAddressRange::new();
        ip_addr_range.generate(&result);
        println!("IP Addresses {}", ip_addr_range.ip_count());

        let ip_pinger = IpPinger::new(ip_addr_range.ip_addresses, 1, 1);

        let results = ip_pinger.ping_in_range();

        println!("Working IPs {:?} ", results)
    
    }
}
