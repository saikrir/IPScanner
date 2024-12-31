use IPScanner::{InputIp, IpAddressRange, IpPinger};




fn main() {
    let result = InputIp::parse("192.168.86", 1, 255);
    let mut ip_addr_range = IpAddressRange::new();
    ip_addr_range.generate(&result);
    println!("IP Addresses {}", ip_addr_range.ip_count());

    let ip_pinger = IpPinger::new(ip_addr_range.ip_addresses, 1, 1);

    let results = ip_pinger.ping_in_range();

    println!("Found {}, IPs: {:?} ", results.len(), results);
}