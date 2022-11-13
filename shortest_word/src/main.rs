fn find_short(s: &str) -> u32 {
    s.split(' ').map(str::len).min().unwrap_or(0) as u32
}

fn main() {
    println!("{}", find_short("bitcoin take over the world maybe who knows perhaps"));
}
