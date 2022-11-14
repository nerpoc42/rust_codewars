fn is_valid_ip(ip: &str) -> bool {
    ip.split('.').count() == 4 && 
        ip.split('.')
        .all(|x| {
            (!x.starts_with('0') || x == "0") && x.parse::<u8>().is_ok()
        })
}

fn main() {
    println!("{}", is_valid_ip("123.045.067.089"));
}
