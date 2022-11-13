fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..=end).filter(|x| {
        let mut i = x.clone().abs();
        while i != 0 {
            if i % 10 == 5 {
                return false;
            }
            i = i / 10;
        }
        println!("{}", x);
        true
    }).count() as isize
}

fn main() {
    println!("{}", dont_give_me_five(4, 17));
}
