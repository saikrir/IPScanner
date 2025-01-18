use std::env;

use ipscanner::{get_local_ip_address, ping_ips, InputIp, IpAddressRange};

fn main() {
    let local_ip = get_local_ip_address();

    println!("Your local ip: {local_ip}");

    let result = InputIp::parse(&local_ip, 1, 255);

    let mut ip_addr_range = IpAddressRange::new();
    ip_addr_range.generate(&result);
    println!("IP Addresses to scan {}", ip_addr_range.ip_count());
    let results = ping_ips(ip_addr_range.ip_addresses);
    println!("Found {}, {} ", results.len(), env::consts::OS);

    for ip in results {
        println!("{}", ip)
    }
}
