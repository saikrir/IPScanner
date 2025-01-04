use std::{ops::Range, sync::mpsc, thread, time::{Duration, SystemTime}};

use pinger::{ping, PingOptions, PingResult};

pub struct InputIp {
    ip_triplet: [u8; 3],
    scan_range: Range<u8>,
}

impl InputIp {
    pub fn parse(input_triplet: &str, start: u8, end: u8) -> Self {
        let vec: Vec<u8> = input_triplet
            .split(".")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        let range = start..end;
        Self {
            ip_triplet: [vec[0], vec[1], vec[2]],
            scan_range: range,
        }
    }

    pub fn scan_range(&self) -> Range<u8> {
        self.scan_range.clone()
    }
}

impl ToString for InputIp {
    fn to_string(&self) -> String {
        format!(
            "{}.{}.{}",
            self.ip_triplet[0], self.ip_triplet[1], self.ip_triplet[2]
        )
    }
}

pub struct IpAddressRange {
    pub ip_addresses: Vec<String>,
}

impl IpAddressRange {
    pub fn new() -> Self {
        let v: Vec<String> = Vec::new();
        Self { ip_addresses: v }
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


fn pingOptions(ping_count: u8, interval: Duration, ip_address: &str) -> PingOptions {
    let ping_options = PingOptions::new(ip_address, interval, None);
    ping_options.with_raw_arguments(vec!["-c", ping_count.to_string().as_str()])
}

pub fn ping_ips(ip_addresses: &Vec<String>, ping_cnt: u8) -> Vec<String> {
    let mut ret_val: Vec<String> = vec![];
    for _x in 1..=ping_cnt {
        let start_time = SystemTime::now();
        let (tx, rx) = mpsc::channel::<String>();
        for ip_address in ip_addresses.clone() {
            let local_tx = tx.clone();

            thread::spawn(move || {
                let ping_opts = pingOptions(1, Duration::from_secs(1), &ip_address);

                let stream = ping(ping_opts).expect("Error pinging");

                let message = stream.iter().next().unwrap();

                match message {
                    PingResult::Pong(_, _) => local_tx.send(ip_address.to_owned()).unwrap(),
                    _ => {}
                }
            });
        }
        drop(tx);
        ret_val = rx.iter().collect();
        let elapsed = start_time.elapsed().unwrap();
        println!("Pinged {} ips in {} Secods", ip_addresses.len(), elapsed.as_secs_f64());
    }
    ret_val.sort_by(|s1, s2| s1.cmp(s2));
    ret_val
}



trait Pinger {
    fn ping(&self, ip_address: &str) -> Result<(), &str>;
}

// pub struct IpPinger {
//     ping_interval: Duration,
// }


// impl Pinger for IpPinger {
//     fn ping(&self, ip_address: &str) -> Result<(), &str> {
//         let ping_opts = pingOptions(1, self.ping_interval, ip_address);

//         let stream = ping(ping_opts).expect("Error pinging");

//         let message = stream.iter().next().unwrap();

//         match message {
//             PingResult::Pong(_, _) => Ok(()),
//             _ => Err("Some Error occured"),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_parses() {
        let result = InputIp::parse("192.168.86.127", 1, 255);
        assert_eq!([192, 168, 86], result.ip_triplet);
        println!("{}", result.to_string())
    }

    #[test]
    fn ip_count_is_correct() {
        let result = InputIp::parse("192.168.86.127", 1, 255);
        let mut ip_addr_range = IpAddressRange::new();
        ip_addr_range.generate(&result);
        assert_eq!(254, ip_addr_range.ip_count());
    }

  
}
