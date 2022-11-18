fn last_digit(str1: &str, str2: &str) -> u32 {
    let (base, power) = (str1.chars().last().unwrap(), str2.chars().last().unwrap());

    let (base, power) = (base.to_digit(10).unwrap(), power.to_digit(10).unwrap());

    let (mut digit, mult) = (base % 10, base % 10);

    for _ in 1..power {
        digit = (digit * mult) % 10;
    }

    digit
}

fn main() {
    println!("{}", last_digit("13", "2"));
}
