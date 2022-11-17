fn make_readable(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = seconds / 60 % 60; 
    let seconds = seconds - minutes * 60 - hours * 3600;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    println!("{}", make_readable(1200));
}
