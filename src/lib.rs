use pinger::{ping, PingOptions, PingResult};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::thread::JoinHandle;
use std::{
    ops::Range,
    sync::mpsc,
    thread,
    time::{Duration, SystemTime},
};

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

fn ping_options(ping_count: u8, interval: Duration, ip_address: &str) -> PingOptions {
    let ping_options = PingOptions::new(ip_address, interval, None);
    if cfg!(windows) {
        ping_options.with_raw_arguments(vec!["-n", ping_count.to_string().as_str()])
    } else {
        ping_options.with_raw_arguments(vec!["-c", ping_count.to_string().as_str()])
    }
}

pub fn ping_ips(ip_addresses: &Vec<String>, ping_cnt: u8) -> Vec<String> {
    let start_time = SystemTime::now();
    let mut final_set = HashSet::new();
    let mut join_handles: Vec<JoinHandle<Vec<String>>> = vec![];
    for _x in 1..=ping_cnt {
        let ip_addresses = ip_addresses.clone();
        join_handles.push(thread::spawn(|| perform_ping_async(ip_addresses)));
    }

    for handle in join_handles {
        let ip_addresses = handle.join().unwrap();
        for ip_address in ip_addresses {
            final_set.insert(ip_address);
        }
    }

    let elapsed = start_time.elapsed().unwrap();
    println!(
        "Pinged {} ips in {} Seconds",
        ip_addresses.len(),
        elapsed.as_secs_f64()
    );

    let mut v: Vec<String> = final_set.into_iter().collect();
    v.sort_by(|x: &String, x1: &String| compare_ips::<&String>(x, x1));
    v
}

fn get_last_octet_for_ip(ip_address: &str) -> u8 {
    ip_address.split(".").collect::<Vec<&str>>()[3]
        .parse::<u8>()
        .unwrap()
}
fn compare_ips<T: AsRef<str>>(ip_address1: &str, ip_address2: &str) -> Ordering {
    get_last_octet_for_ip(ip_address1.as_ref()).cmp(&get_last_octet_for_ip(ip_address2.as_ref()))
}

fn perform_ping_async(ip_addresses: Vec<String>) -> Vec<String> {
    let (tx, rx) = mpsc::channel::<String>();
    for ip_address in ip_addresses {
        let local_tx = tx.clone();
        thread::spawn(move || {
            let ping_opts = ping_options(1, Duration::from_secs(2), &ip_address);

            let stream = ping(ping_opts).expect("Error pinging");

            let message = stream.iter().next().unwrap();

            match message {
                PingResult::Pong(_, _) => local_tx.send(ip_address.to_owned()).unwrap(),
                _ => {}
            }
        });
    }
    drop(tx);
    rx.iter().collect()
}

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

    #[test]
    fn test_sort() {
        let mut p = vec![
            "192.168.86.31",
            "192.168.86.36",
            "192.168.86.53",
            "192.168.86.33",
            "192.168.86.34",
            "192.168.86.216",
            "192.168.86.35",
            "192.168.86.140",
            "192.168.86.72",
            "192.168.86.48",
            "192.168.86.200",
            "192.168.86.23",
            "192.168.86.122",
            "192.168.86.204",
            "192.168.86.1",
            "192.168.86.40",
            "192.168.86.62",
            "192.168.86.25",
            "192.168.86.42",
            "192.168.86.24",
            "192.168.86.130",
            "192.168.86.250",
            "192.168.86.55",
            "192.168.86.43",
            "192.168.86.37",
            "192.168.86.29",
            "192.168.86.2",
        ];
        p.sort_by(|x: &&str, x1: &&str| compare_ips::<&str>(x, x1));
        println!("Sorted {:?}", p)
    }
}
