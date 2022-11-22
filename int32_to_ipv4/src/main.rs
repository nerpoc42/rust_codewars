use std::net::Ipv4Addr;

fn int32_to_ip(int: u32) -> String {
    format!("{}.{}.{}.{}", int >> 24 & 0xFF, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
}

#[allow(dead_code)]
fn int32_to_ip_better(int: u32) -> String {
    Ipv4Addr::from(int).to_string()
}

fn main() {
    println!("{}", int32_to_ip(2149583361));
}
