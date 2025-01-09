use std::env;

use ipscanner::{ping_ips, InputIp, IpAddressRange};

fn main() {
    let result = InputIp::parse("192.168.86", 1, 255);
    let mut ip_addr_range = IpAddressRange::new();
    ip_addr_range.generate(&result);
    println!("IP Addresses to scan {}", ip_addr_range.ip_count());
    let results = ping_ips(&ip_addr_range.ip_addresses, 2);
    println!("Found {}, {} ", results.len(), env::consts::OS);

    for ip in results {
        println!("{}", ip)
    }
}
// resolve nslookup
// fail safe if ping command was not available
