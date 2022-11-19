fn order_weight(s: &str) -> String {
    let digit_sum = |mut num| {
        let mut sum = 0usize;
        while num != 0 {
            sum = sum + num % 10;
            num = num / 10;
        }
        sum
    };

    let mut weights: Vec<&str> = s.split(' ').collect();
    weights.sort_by(|a, b| {
        digit_sum(a.parse::<usize>().unwrap())
            .cmp(&digit_sum(b.parse::<usize>().unwrap()))
            .then(a.cmp(b))
    });
    weights.join(" ")
}

#[allow(dead_code)]
fn order_weight_better(s: &str) -> String {
  let mut numbers = s.split_whitespace().collect::<Vec<_>>();
  numbers.sort();
  numbers.sort_by_key(|s| s.chars().flat_map(|c| c.to_digit(10)).sum::<u32>());
  numbers.join(" ")
}

fn main() {
    println!("{}", order_weight("2000 10003 1234000 44444444 9999 11 11 22 123"));
}
