fn is_perfect_power(n: u64) -> Option<(u64, u32)> {
    for base in 2..=n/2 {
        let power = (n as f64).log(base as f64).round() as u32;
        if power == 0 {
            break
        }

        if base.pow(power) == n {
            return Some((base, power));
        }
    }
    None
}

fn main() {
    println!("{:?}", is_perfect_power(243));
}
