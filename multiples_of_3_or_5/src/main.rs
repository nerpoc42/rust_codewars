fn solution(num: i32) -> i32 {
    if num < 3 {
        return 0;
    }

    let n = num - 1;
    (3 * (n / 3) * (n / 3 + 1) 
        + 5 * (n / 5) * (n / 5 + 1)
        - 15 * (n / 15) * (n / 15 + 1)) / 2
}

fn main() {
    println!("{}", solution(10));
}
