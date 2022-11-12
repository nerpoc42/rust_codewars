fn solution(s: &str) -> Vec<String> {
    let mut iter = s.chars();
    let mut res = vec![];

    while let Some(item) = iter.next() {
        res.push(format!("{}{}", item, iter.next().unwrap_or('_')));
    }

    res
}

fn main() {
    println!("{:?}", solution("abc"));
}
