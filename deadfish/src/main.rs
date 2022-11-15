fn parse(code: &str) -> Vec<i32> {
    let mut result = vec![];
    let mut num = 0i32;

    code.chars().for_each(|x| {
        match x {
            'i' => num = num + 1,
            'd' => num = num - 1,
            's' => num = num * num,
            'o' => result.push(num),
            _ => {}
        }
    });

    result
}

fn main() {
    println!("{:?}", parse("iiisdoso"));
}
