use std::net::Ipv4Addr;

#[test]
fn name() {
    let st: String = String::from("127.0.0.1");
    let mut parse_ip = st.parse::<Ipv4Addr>().unwrap();
    let prip: u32 = parse_ip.try_into().unwrap();
    println!("{:?}", prip);

    let cidr = String::from("127.0.0.1/24");
    let cidr_parts: Vec<&str> = cidr.split("/").collect();
    let cidr_ip = cidr_parts[0].parse::<Ipv4Addr>().unwrap();
    let cidr_prefix_len = cidr_parts[1].parse::<u8>().unwrap();

    let cidr_mask = (!0u32).checked_shr(cidr_prefix_len as u32).unwrap_or(0);
    println!("cidr_mask: {:?}", cidr_mask);
    let cidr_net = u32::from_be_bytes(cidr_ip.octets()) & cidr_mask;
    let parsed_ip_net = u32::from_be_bytes(parse_ip.octets()) & cidr_mask;
    if cidr_net == parsed_ip_net {
        println!("{} is in the CIDR {}", st, cidr);
    } else {
        println!("{} is not in the CIDR {}", st, cidr);
    }
}
