use ping::dgramsock::ping;
use rand::random;
use std::{net::IpAddr, time::Duration};

fn main() {
    let ip_prefix = "192.168.86";

    let comp: Vec<u8> = ip_prefix
        .split(".")
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    let mut working_ips: Vec<IpAddr> = vec![];
    let timeout = Duration::from_secs(1);

    for ip_addr in get_local_ip_addresses(1, 40, &comp) {
        let working_ip = match ping(
            ip_addr,
            Some(timeout),
            Some(166),
            Some(3),
            Some(5),
            Some(&random()),
        ) {
            Ok(()) => ip_addr,
            Err(_) => {
                continue;
            }
        };
        working_ips.push(working_ip);
    }

    println!("Found {} ips {:#?} ", working_ips.len(), working_ips)
}

fn get_local_ip_addresses(begin_index: u8, end_index: u8, prefix: &Vec<u8>) -> Vec<IpAddr> {
    let mut ip_addresses: Vec<IpAddr> = vec![];

    for last_octet in begin_index..end_index {
        let ip_addr = IpAddr::from([prefix[0], prefix[1], prefix[2], last_octet]);
        ip_addresses.push(ip_addr);
    }
    return ip_addresses;
}
